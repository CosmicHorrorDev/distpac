use std::{convert::TryFrom, path::Path};

use crate::{error::PackageError, manifest::Manifest};

pub mod error;
pub mod manifest;
mod utils;

pub fn validate_package(package_path: &Path) -> Result<(), PackageError> {
    // Build all the paths expected from a package
    let manifest_file = package_path.join("manifest.yaml");
    let assets_dir = package_path.join("assets");
    let scripts_dir = package_path.join("scripts");
    let install_script = scripts_dir.join("install.sh");
    let uninstall_script = scripts_dir.join("uninstall.sh");

    // And validate their attributes
    utils::validate_file(&manifest_file)?;
    utils::validate_dir(&assets_dir)?;
    utils::validate_dir(&scripts_dir)?;
    utils::validate_file(&install_script)?;
    utils::validate_file(&uninstall_script)?;
    Manifest::try_from(manifest_file.as_path())?;

    Ok(())
}
