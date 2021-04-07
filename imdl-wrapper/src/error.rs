use std::{io, path::PathBuf};

#[derive(thiserror::Error, Debug)]
pub enum TorrentError {
    #[error("Encountered IO error {0}")]
    IoError(#[from] io::Error),
    #[error("Failed to create the torrent file")]
    FailedCreatingTorrent,
    #[error("Failed to generate the torrent's magnet link")]
    FailedGeneratingMagnent,
    #[error("Failed to extract the torrent's info hash")]
    FailedExtractingInfoHash,
    #[error("Missing torrent file expected at {0}")]
    MissingTorrentFile(PathBuf),
    #[error("Missing input path expected at {0}")]
    MissingInputPath(PathBuf),
    #[error("Destination path missing or not dir at {0}")]
    DestinationMissingOrNotDir(PathBuf),
}
