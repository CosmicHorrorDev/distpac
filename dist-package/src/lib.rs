use std::{convert::TryFrom, path::Path};

use crate::manifest::Manifest;

pub mod error;
pub mod manifest;

pub fn valid_package(package_path: &Path) -> bool {
    let manifest_result = Manifest::try_from(package_path.join("manifest.yaml").as_path());

    let assets_dir = package_path.join("assets");
    let scripts_dir = package_path.join("scripts");
    let install_script = scripts_dir.join("install.sh");
    let uninstall_script = scripts_dir.join("uninstall.sh");

    manifest_result.is_ok()
        && assets_dir.is_dir()
        && install_script.is_file()
        && uninstall_script.is_file()
}
