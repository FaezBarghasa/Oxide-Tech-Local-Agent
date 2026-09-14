use anyhow::{anyhow, Context, Result};
use memmap2::Mmap;
use safetensors::tensor::{SafeTensorError, SafeTensors};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::File;
use std::path::{Path, PathBuf};
use tracing::{debug, info};

/// Tensor metadata extracted from a `.safetensors` header.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TensorMeta {
    pub name: String,
    pub dtype: String,
    pub shape: Vec<usize>,
    pub num_elements: usize,
    pub byte_size: usize,
}

/// Model info summary for loaded or inspected `.safetensors` model files.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SafetensorModelSummary {
    pub path: PathBuf,
    pub total_tensors: usize,
    pub total_parameters: usize,
    pub total_bytes: usize,
    pub tensor_names: Vec<String>,
}

/// Zero-copy, memory-mapped loader and inspector for `.safetensors` weights and checkpoints.
pub struct SafetensorModelLoader {
    model_path: PathBuf,
    mmap: Mmap,
}

impl SafetensorModelLoader {
    /// Open and memory-map a `.safetensors` file.
    pub fn open<P: AsRef<Path>>(path: P) -> Result<Self> {
        let path_buf = path.as_ref().to_path_buf();
        if !path_buf.exists() {
            return Err(anyhow!("Safetensors file not found at {:?}", path_buf));
        }

        let file = File::open(&path_buf)
            .with_context(|| format!("Failed to open safetensors file at {:?}", path_buf))?;

        // Memory-map the binary weights for instant zero-copy tensor slicing
        let mmap = unsafe {
            Mmap::map(&file)
                .with_context(|| format!("Failed to mmap safetensors file at {:?}", path_buf))?
        };

        info!(
            "Successfully memory-mapped safetensors model: {:?} ({} bytes)",
            path_buf,
            mmap.len()
        );

        Ok(Self {
            model_path: path_buf,
            mmap,
        })
    }

    /// Retrieve the parsed `SafeTensors` view over the memory map.
    pub fn tensors(&self) -> Result<SafeTensors<'_>, SafeTensorError> {
        SafeTensors::deserialize(&self.mmap)
    }

    /// Inspect all tensor names, shapes, data types, and total parameters.
    pub fn summarize(&self) -> Result<SafetensorModelSummary> {
        let tensors = self
            .tensors()
            .map_err(|e| anyhow!("Failed to deserialize safetensors: {:?}", e))?;

        let names = tensors.names();
        let mut total_params: usize = 0;
        let mut tensor_names = Vec::with_capacity(names.len());

        for name in &names {
            if let Ok(tensor) = tensors.tensor(name) {
                let params: usize = tensor.shape().iter().product();
                total_params += params;
                tensor_names.push((*name).to_string());
            }
        }

        Ok(SafetensorModelSummary {
            path: self.model_path.clone(),
            total_tensors: names.len(),
            total_parameters: total_params,
            total_bytes: self.mmap.len(),
            tensor_names,
        })
    }

    /// Extract a specific tensor's raw byte slice by name.
    pub fn get_tensor_data(&self, tensor_name: &str) -> Result<&[u8]> {
        let tensors = self
            .tensors()
            .map_err(|e| anyhow!("Failed to deserialize safetensors: {:?}", e))?;

        let view = tensors
            .tensor(tensor_name)
            .map_err(|e| anyhow!("Tensor '{}' not found: {:?}", tensor_name, e))?;

        Ok(view.data())
    }

    /// Discover and list all `.safetensors` files in a given directory (e.g. HuggingFace snapshot).
    pub fn discover_models_in_dir<P: AsRef<Path>>(dir: P) -> Result<Vec<PathBuf>> {
        let mut found = Vec::new();
        let dir_ref = dir.as_ref();
        if dir_ref.is_dir() {
            for entry in std::fs::read_dir(dir_ref)? {
                let entry = entry?;
                let path = entry.path();
                if path.extension().and_then(|ext| ext.to_str()) == Some("safetensors") {
                    found.push(path);
                }
            }
        }
        Ok(found)
    }
}
