use std::{io, path::PathBuf};

#[derive(thiserror::Error, Debug)]
pub enum PackageError {
    #[error("Encountered IO error when dealing with package Error: {0}")]
    Io(#[from] io::Error),
    #[error("Manifest isn't correctly formatted Error: {0}")]
    InvalidManifest(#[from] serde_yaml::Error),
    #[error("Missing expected file {0}")]
    MissingFile(PathBuf),
    #[error("Missing expected directory {0}")]
    MissingDir(PathBuf),
}
