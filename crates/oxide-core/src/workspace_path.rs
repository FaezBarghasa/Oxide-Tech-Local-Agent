use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use thiserror::Error;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum PathError {
    #[error("Invalid root directory: {0}")]
    InvalidRoot(String),
    #[error("Path traversal detected: path '{target}' escapes workspace root '{root}'")]
    PathTraversal { target: String, root: String },
}

/// Enforces containment of file paths within an authoritative workspace root.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkspacePath {
    root: PathBuf,
    resolved: PathBuf,
}

impl WorkspacePath {
    /// Constructs and validates a path relative to root, rejecting any traversal attempt.
    pub fn new(root: impl AsRef<Path>, untrusted_rel: impl AsRef<Path>) -> Result<Self, PathError> {
        let root_ref = root.as_ref();
        let canonical_root = root_ref
            .canonicalize()
            .map_err(|e| PathError::InvalidRoot(format!("{}: {}", root_ref.display(), e)))?;

        let untrusted = untrusted_rel.as_ref();
        let combined = if untrusted.is_absolute() {
            untrusted.to_path_buf()
        } else {
            canonical_root.join(untrusted)
        };

        // Resolve path even if the final leaf node does not exist yet (for write targets)
        let resolved = if combined.exists() {
            combined
                .canonicalize()
                .map_err(|e| PathError::InvalidRoot(e.to_string()))?
        } else if let Some(parent) = combined.parent() {
            if parent.exists() {
                let canonical_parent = parent
                    .canonicalize()
                    .map_err(|e| PathError::InvalidRoot(e.to_string()))?;
                if let Some(file_name) = combined.file_name() {
                    canonical_parent.join(file_name)
                } else {
                    canonical_parent
                }
            } else {
                // Normalize manually if parent doesn't exist
                normalize_path(&combined)
            }
        } else {
            normalize_path(&combined)
        };

        if !resolved.starts_with(&canonical_root) {
            return Err(PathError::PathTraversal {
                target: resolved.display().to_string(),
                root: canonical_root.display().to_string(),
            });
        }

        Ok(Self {
            root: canonical_root,
            resolved,
        })
    }

    pub fn root(&self) -> &Path {
        &self.root
    }

    pub fn as_path(&self) -> &Path {
        &self.resolved
    }

    pub fn to_path_buf(&self) -> PathBuf {
        self.resolved.clone()
    }
}

fn normalize_path(path: &Path) -> PathBuf {
    let mut components = Vec::new();
    for c in path.components() {
        match c {
            std::path::Component::ParentDir => {
                components.pop();
            }
            std::path::Component::CurDir => {}
            _ => components.push(c),
        }
    }
    components.into_iter().collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_valid_contained_path() {
        let dir = tempdir().unwrap();
        let ws = WorkspacePath::new(dir.path(), "src/main.rs").unwrap();
        assert!(ws.as_path().starts_with(dir.path().canonicalize().unwrap()));
    }

    #[test]
    fn test_path_traversal_rejected() {
        let dir = tempdir().unwrap();
        let err = WorkspacePath::new(dir.path(), "../../etc/passwd").unwrap_err();
        match err {
            PathError::PathTraversal { .. } => {}
            _ => panic!("Expected PathTraversal error"),
        }
    }
}
