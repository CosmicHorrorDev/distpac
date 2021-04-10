use std::path::Path;

use crate::error::PackageError;

pub fn validate_file(path: &Path) -> Result<(), PackageError> {
    if path.is_file() {
        Ok(())
    } else {
        Err(PackageError::MissingFile(path.to_owned()))
    }
}

pub fn validate_dir(path: &Path) -> Result<(), PackageError> {
    if path.is_dir() {
        Ok(())
    } else {
        Err(PackageError::MissingDir(path.to_owned()))
    }
}
