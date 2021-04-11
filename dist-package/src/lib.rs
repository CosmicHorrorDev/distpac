use fs_extra::dir;
use imdl_wrapper::Torrent;

use std::{
    convert::TryFrom,
    path::{Path, PathBuf},
};

use crate::{error::PackageError, manifest::Manifest};

pub mod error;
pub mod manifest;
mod utils;

#[derive(Default)]
pub struct PackageOpts {
    torrent_dir: PathBuf,
    package_dir: PathBuf,
}

impl PackageOpts {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn torrent_dir(mut self, torrent_dir: PathBuf) -> Self {
        self.torrent_dir = torrent_dir;
        self
    }

    pub fn package_dir(mut self, package_dir: PathBuf) -> Self {
        self.package_dir = package_dir;
        self
    }
}

pub struct NewPackage {
    manifest: Manifest,
    package_path: PathBuf,
}

impl NewPackage {
    pub fn new(package_path: PathBuf) -> Result<Self, PackageError> {
        Self::validate(&package_path)?;
        let manifest = Manifest::try_from(package_path.as_path())?;
        Ok(Self {
            manifest,
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

pub struct AddedPackage {
    manifest: Manifest,
    installed_path: PathBuf,
    torrent: Torrent,
}

impl AddedPackage {
    pub fn new(new_package: NewPackage) -> Result<Self, PackageError> {
        Self::new_with_opts(new_package, PackageOpts::default())
    }

    pub fn new_with_opts(new_package: NewPackage, opts: PackageOpts) -> Result<Self, PackageError> {
        let NewPackage {
            manifest,
            package_path: old_package_path,
        } = new_package;
        let PackageOpts {
            torrent_dir,
            package_dir: new_package_path,
        } = opts;

        // And move the package to the installed location
        fs_extra::move_items(
            &[old_package_path],
            &new_package_path,
            &dir::CopyOptions::default(),
        )?;

        // Create the torrent
        let torrent = Torrent::create(&new_package_path, &torrent_dir)?;

        Ok(Self {
            manifest,
            installed_path: new_package_path,
            torrent,
        })
    }

    pub fn manifest(&self) -> &Manifest {
        &self.manifest
    }

    pub fn installed_path(&self) -> &Path {
        &self.installed_path
    }

    pub fn torrent(&self) -> &Torrent {
        &self.torrent
    }
}
