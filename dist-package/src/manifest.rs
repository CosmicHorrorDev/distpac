use serde::Deserialize;
use serde_yaml;

use std::{convert::TryFrom, fs::File, path::Path};

use crate::error::ManifestError;

#[derive(Deserialize, Debug)]
pub struct Manifest {
    name: String,
    version: String,
}

impl TryFrom<&Path> for Manifest {
    type Error = ManifestError;

    fn try_from(manifest_path: &Path) -> Result<Self, Self::Error> {
        let file = File::open(manifest_path)?;
        let manifest = serde_yaml::from_reader(file)?;

        Ok(manifest)
    }
}
