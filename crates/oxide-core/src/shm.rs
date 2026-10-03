use crate::OxideError;
use std::os::unix::io::RawFd;

/// POSIX anonymous sealed shared memory allocator for zero-copy IPC.
pub struct SealedSharedMemory {
    pub fd: RawFd,
    pub size_bytes: usize,
    pub name: String,
}

impl SealedSharedMemory {
    /// Allocate an anonymous RAM-backed sealed memory descriptor.
    pub fn allocate(name: &str, size_bytes: usize) -> Result<Self, OxideError> {
        #[cfg(target_os = "linux")]
        {
            use std::ffi::CString;

            let c_name = CString::new(name)
                .map_err(|e| OxideError::Kernel(format!("Invalid SHM name: {}", e)))?;

            // MFD_CLOEXEC = 0x0001, MFD_ALLOW_SEALING = 0x0002
            let fd = unsafe {
                libc::syscall(
                    libc::SYS_memfd_create,
                    c_name.as_ptr(),
                    0x0001u32 | 0x0002u32, // MFD_CLOEXEC | MFD_ALLOW_SEALING
                ) as RawFd
            };

            if fd < 0 {
                return Err(OxideError::Kernel(format!(
                    "memfd_create failed: {}",
                    std::io::Error::last_os_error()
                )));
            }

            // Truncate to size
            let trunc_res = unsafe { libc::ftruncate(fd, size_bytes as libc::off_t) };
            if trunc_res < 0 {
                unsafe { libc::close(fd) };
                return Err(OxideError::Kernel(format!(
                    "ftruncate failed: {}",
                    std::io::Error::last_os_error()
                )));
            }

            // Apply seals: F_SEAL_SHRINK (0x0002) | F_SEAL_GROW (0x0004) | F_SEAL_SEAL (0x0001)
            // F_ADD_SEALS = 1033 (on Linux x86_64/arm64)
            let seals = 0x0001 | 0x0002 | 0x0004;
            let seal_res = unsafe { libc::fcntl(fd, 1033, seals) };
            if seal_res < 0 {
                unsafe { libc::close(fd) };
                return Err(OxideError::Kernel(format!(
                    "fcntl sealing failed: {}",
                    std::io::Error::last_os_error()
                )));
            }

            Ok(Self {
                fd,
                size_bytes,
                name: name.to_string(),
            })
        }

        #[cfg(not(target_os = "linux"))]
        {
            Err(OxideError::Kernel(
                "SealedSharedMemory is supported on Linux".to_string(),
            ))
        }
    }
}

impl Drop for SealedSharedMemory {
    fn drop(&mut self) {
        if self.fd >= 0 {
            unsafe {
                libc::close(self.fd);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[cfg(target_os = "linux")]
    fn test_sealed_shared_memory_allocation() {
        let shm = SealedSharedMemory::allocate("test_oxide_shm", 1024 * 1024)
            .expect("Failed to allocate sealed shm");
        assert!(shm.fd >= 0);
        assert_eq!(shm.size_bytes, 1024 * 1024);
    }
}
