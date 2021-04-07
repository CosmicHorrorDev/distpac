use std::io;

#[derive(thiserror::Error, Debug)]
pub enum ManifestError {
    #[error("Encountered IO error when dealing with manifest file")]
    Io(#[from] io::Error),
    #[error("Failed deserializing manifest file")]
    Deserialization(#[from] serde_yaml::Error),
}
