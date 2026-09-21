use memmap2::{Mmap, MmapOptions};
use oxide_core::OxideError;
use std::fs::File;
use std::path::Path;
use std::ptr::NonNull;
use std::sync::Arc;

/// GGUF tensor quantization formats including mixed-precision Importance Matrix (IMatrix) types.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(non_camel_case_types)]
pub enum GgufTensorType {
    F32,
    F16,
    BF16,
    Q4_0,
    Q4_1,
    Q4_K, // Base 4-bit block
    Q5_0,
    Q5_1,
    Q5_K, // 5-bit block (used for critical attn_v / ffn_down layers in UD-Q4_K_XL)
    Q6_K, // 6-bit block (used for high-importance matrix weights)
    Q8_0,
    IQ4_NL,
    IQ4_XS,
    IQ3_XXS,
    IQ3_XS,
    IQ2_XXS,
}

/// Metadata describing an individual tensor inside an IMatrix / GGUF model file.
#[derive(Debug, Clone)]
pub struct GgufTensorInfo {
    pub name: String,
    pub tensor_type: GgufTensorType,
    pub shape: Vec<usize>,
    pub offset: usize,
    pub size_bytes: usize,
}

/// Kernel memory access pattern advice for memory-mapped model files.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MemoryAdvice {
    /// Sequential read pattern (e.g. streaming initial weight loading).
    Sequential,
    /// Random access pattern (e.g. KV cache paging, sparse MoE activation).
    Random,
    /// Pre-fault / page in data immediately into physical RAM.
    WillNeed,
    /// Request Transparent Huge Pages (THP) allocation to reduce TLB overhead.
    HugePages,
}

/// Zero-copy memory-mapped model container.
pub struct MmapModel {
    mmap: Arc<Mmap>,
    file_size: usize,
}

impl MmapModel {
    /// Memory map a model file from disk into virtual address space without copying.
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self, OxideError> {
        let file = File::open(path.as_ref())
            .map_err(|e| OxideError::Engine(format!("Failed to open model file: {}", e)))?;

        let metadata = file
            .metadata()
            .map_err(|e| OxideError::Engine(format!("Failed to read file metadata: {}", e)))?;
        let file_size = metadata.len() as usize;

        if file_size == 0 {
            return Err(OxideError::Engine("Cannot mmap empty file".to_string()));
        }

        let mmap = unsafe {
            MmapOptions::new()
                .map(&file)
                .map_err(|e| OxideError::Engine(format!("mmap call failed: {}", e)))?
        };

        Ok(Self {
            mmap: Arc::new(mmap),
            file_size,
        })
    }

    /// Total mapped byte length.
    pub fn len(&self) -> usize {
        self.file_size
    }

    /// Check if mapped region is empty.
    pub fn is_empty(&self) -> bool {
        self.file_size == 0
    }

    /// Advise the Linux kernel on paging strategies for this model region.
    pub fn advise(&self, advice: MemoryAdvice) -> Result<(), OxideError> {
        #[cfg(target_os = "linux")]
        unsafe {
            let addr = self.mmap.as_ptr() as *mut libc::c_void;
            let len = self.file_size;

            let res = match advice {
                MemoryAdvice::Sequential => {
                    libc::posix_madvise(addr, len, libc::POSIX_MADV_SEQUENTIAL)
                }
                MemoryAdvice::Random => libc::posix_madvise(addr, len, libc::POSIX_MADV_RANDOM),
                MemoryAdvice::WillNeed => libc::posix_madvise(addr, len, libc::POSIX_MADV_WILLNEED),
                MemoryAdvice::HugePages => {
                    // MADV_HUGEPAGE is a Linux-specific madvise flag
                    libc::madvise(addr, len, libc::MADV_HUGEPAGE)
                }
            };

            if res != 0 {
                return Err(OxideError::Kernel(format!(
                    "madvise({:?}) failed with code {}",
                    advice, res
                )));
            }
        }
        Ok(())
    }

    /// Extract a zero-copy tensor slice with strict boundary and alignment checks.
    pub fn get_tensor_slice(
        &self,
        offset: usize,
        size_bytes: usize,
        alignment: usize,
    ) -> Result<TensorSlice, OxideError> {
        if offset + size_bytes > self.file_size {
            return Err(OxideError::Engine(format!(
                "Tensor bounds out of range: offset {} + size {} > total {}",
                offset, size_bytes, self.file_size
            )));
        }

        let ptr = unsafe { self.mmap.as_ptr().add(offset) };

        // Verify alignment invariant
        if alignment > 1 && !(ptr as usize).is_multiple_of(alignment) {
            return Err(OxideError::Engine(format!(
                "Tensor pointer {:p} does not satisfy alignment requirement of {} bytes",
                ptr, alignment
            )));
        }

        let non_null = NonNull::new(ptr as *mut u8).ok_or_else(|| {
            OxideError::Engine(
                "Null pointer encountered during tensor slice extraction".to_string(),
            )
        })?;

        Ok(TensorSlice {
            _owner: self.mmap.clone(),
            ptr: non_null,
            len: size_bytes,
        })
    }
}

/// A zero-copy slice of memory-mapped model weights tied to the underlying `Mmap` lifetime.
#[derive(Clone)]
pub struct TensorSlice {
    _owner: Arc<Mmap>,
    ptr: NonNull<u8>,
    len: usize,
}

// TensorSlice is Send and Sync because the underlying Mmap is read-only and immutable.
unsafe impl Send for TensorSlice {}
unsafe impl Sync for TensorSlice {}

impl TensorSlice {
    /// Return the raw pointer to tensor data for FFI and CUDA kernels.
    pub fn as_ptr(&self) -> *const u8 {
        self.ptr.as_ptr()
    }

    /// Read data as a standard byte slice.
    pub fn as_slice(&self) -> &[u8] {
        unsafe { std::slice::from_raw_parts(self.ptr.as_ptr(), self.len) }
    }

    /// Byte length of the tensor slice.
    pub fn len(&self) -> usize {
        self.len
    }

    /// Check if slice is empty.
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn test_mmap_tensor_lifecycle() {
        let mut temp = NamedTempFile::new().unwrap();
        let sample_weights: Vec<u8> = (0..1024).map(|i| (i % 256) as u8).collect();
        temp.write_all(&sample_weights).unwrap();
        temp.flush().unwrap();

        let model = MmapModel::from_file(temp.path()).unwrap();
        assert_eq!(model.len(), 1024);

        // Advise kernel
        assert!(model.advise(MemoryAdvice::WillNeed).is_ok());

        // Extract aligned slice
        let tensor = model.get_tensor_slice(64, 128, 1).unwrap();
        assert_eq!(tensor.len(), 128);
        assert_eq!(tensor.as_slice(), &sample_weights[64..192]);
        assert_eq!(tensor.as_ptr(), tensor.as_slice().as_ptr());

        // Out of bounds check
        let oob = model.get_tensor_slice(1000, 100, 1);
        assert!(oob.is_err());
    }
}
