use std::{io, path::PathBuf};

#[derive(thiserror::Error, Debug)]
pub enum PackageError {
    // For some reason fs_extra doesn't provide a `impl From<FsError> for io::Error`, which is
    // annoying since it's nearly the same
    #[error("Encountered Filesystem error when dealing with package Error: {0}")]
    Fs(#[from] fs_extra::error::Error),
    #[error("Encountered IO error when dealing with package Error: {0}")]
    Io(#[from] io::Error),
    #[error("Manifest isn't correctly formatted Error: {0}")]
    InvalidManifest(#[from] serde_yaml::Error),
    #[error("Missing expected file {0}")]
    MissingFile(PathBuf),
    #[error("Missing expected directory {0}")]
    MissingDir(PathBuf),
    #[error("Error creating the torrent file")]
    Torrent(#[from] imdl_wrapper::error::TorrentError),
}

#[derive(thiserror::Error, Debug)]
pub enum ParseVersionError {
    #[error("Expected three values, but got {0} instead")]
    InvalidValuesCount(usize),
    #[error("Expected valid u16, but got {0} instead")]
    InvalidValue(String),
}
