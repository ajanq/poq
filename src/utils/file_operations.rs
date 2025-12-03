use crate::error::ProjectError;
use log::{debug, error};
use std::fs;
use std::io;
use std::path::Path;
use tokio::fs as tokio_fs;

pub fn create_directory<P: AsRef<Path>>(path: P) -> Result<(), ProjectError> {
    debug!("Creating directory: {:?}", path.as_ref());
    fs::create_dir_all(&path).map_err(|e| {
        error!(
            "Failed to create directory '{}': {}",
            path.as_ref().display(),
            e
        );
        ProjectError::Io(io::Error::new(
            e.kind(),
            format!(
                "Failed to create directory '{}': {}",
                path.as_ref().display(),
                e
            ),
        ))
    })
}

#[allow(dead_code)]
pub fn copy_file<P: AsRef<Path>, Q: AsRef<Path>>(from: P, to: Q) -> Result<u64, ProjectError> {
    fs::copy(&from, &to).map_err(|e| {
        error!(
            "Failed to copy file from '{}' to '{}': {}",
            from.as_ref().display(),
            to.as_ref().display(),
            e
        );
        ProjectError::Io(e)
    })
}

pub fn write_file<P: AsRef<Path>, C: AsRef<[u8]>>(path: P, content: C) -> Result<(), ProjectError> {
    fs::write(&path, content).map_err(|e| {
        error!("Failed to write file '{}': {}", path.as_ref().display(), e);
        ProjectError::Io(e)
    })
}

#[allow(dead_code)]
pub async fn async_create_directory<P: AsRef<Path>>(path: P) -> Result<(), ProjectError> {
    let path_ref = path.as_ref();
    tokio_fs::create_dir_all(path_ref).await.map_err(|e| {
        error!("Failed to create directory '{}': {}", path_ref.display(), e);
        ProjectError::Io(e)
    })
}

#[allow(dead_code)]
pub async fn async_copy_file<P: AsRef<Path>, Q: AsRef<Path>>(
    from: P,
    to: Q,
) -> Result<u64, ProjectError> {
    let from_ref = from.as_ref();
    let to_ref = to.as_ref();
    tokio_fs::copy(from_ref, to_ref).await.map_err(|e| {
        error!(
            "Failed to copy file from '{}' to '{}': {}",
            from_ref.display(),
            to_ref.display(),
            e
        );
        ProjectError::Io(e)
    })
}

#[allow(dead_code)]
pub async fn async_write_file<P: AsRef<Path>, C: AsRef<[u8]>>(
    path: P,
    content: C,
) -> Result<(), ProjectError> {
    let path_ref = path.as_ref();
    tokio_fs::write(path_ref, content).await.map_err(|e| {
        error!("Failed to write file '{}': {}", path_ref.display(), e);
        ProjectError::Io(e)
    })
}
