use fs_extra::dir;

use std::{
    convert::TryFrom,
    fs,
    path::{Path, PathBuf},
};

use crate::{
    error::PackageError,
    manifest::{Manifest, Version},
};

pub use imdl_wrapper::Torrent;
pub mod error;
pub mod manifest;
mod utils;

pub struct NewPackage {
    name: String,
    version: Version,
    package_path: PathBuf,
}

impl NewPackage {
    pub fn new(package_path: PathBuf) -> Result<Self, PackageError> {
        Self::validate(&package_path)?;
        let Manifest { name, version } =
            Manifest::try_from(package_path.join("manifest.yaml").as_path())?;
        Ok(Self {
            name,
            version,
            package_path,
        })
    }

    fn validate(package_path: &Path) -> Result<(), PackageError> {
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
}

#[derive(Clone, Debug)]
pub struct AddedPackage {
    pub name: String,
    pub version: Version,
    pub installed_path: PathBuf,
    pub torrent: Torrent,
}

impl AddedPackage {
    pub fn new(
        new_package: NewPackage,
        packages_dir: PathBuf,
        torrent_dir: PathBuf,
        announce_url: &str,
    ) -> Result<Self, PackageError> {
        let NewPackage {
            name,
            version,
            package_path: old_package_path,
        } = new_package;

        // Rename the package directory based on the package name and version
        let package_dir_name = format!("{}-{}", name, version);
        let mut package_path = old_package_path.clone();
        package_path.set_file_name(package_dir_name);
        fs::rename(&old_package_path, &package_path)?;

        // And move the package to the installed location
        fs_extra::move_items(
            &[&package_path],
            &packages_dir,
            &dir::CopyOptions::default(),
        )?;

        // Create the torrent
        let package_dir = packages_dir.join(&package_path.file_name().unwrap());
        let torrent = Torrent::create(&package_dir, &torrent_dir, announce_url)?;

        Ok(Self {
            name,
            version,
            installed_path: package_dir,
            torrent,
        })
    }

    pub fn version(&self) -> Version {
        self.version
    }

    pub fn installed_path(&self) -> &Path {
        &self.installed_path
    }

    pub fn torrent(&self) -> &Torrent {
        &self.torrent
    }
}
