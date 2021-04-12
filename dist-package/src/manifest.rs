use serde::Deserialize;
use serde_yaml;

use std::{convert::TryFrom, fs::File, path::Path};

use crate::error::PackageError;

#[derive(Deserialize, Debug)]
pub struct Manifest {
    pub name: String,
    pub version: String,
}

impl TryFrom<&Path> for Manifest {
    type Error = PackageError;

    fn try_from(manifest_path: &Path) -> Result<Self, Self::Error> {
        let file = File::open(manifest_path)?;
        let manifest = serde_yaml::from_reader(file)?;

        Ok(manifest)
    }
}
