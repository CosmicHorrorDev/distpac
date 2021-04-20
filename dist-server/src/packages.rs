use anyhow::Result;
use dist_package::{AddedPackage, NewPackage};
use dist_package_db::database::{DistpacDB, MissingDBAction};

use std::path::PathBuf;

use crate::config::Config;

pub fn add_packages(package_paths: Vec<PathBuf>) -> Result<()> {
    let Config { announce_url } = Config::try_new()?;
    let package_db = DistpacDB::connect(
        &dist_utils::path::package_db_file(),
        MissingDBAction::Create,
    )?;

    // Validate all the new packages first
    let mut new_packages = Vec::with_capacity(package_paths.len());
    for package_path in package_paths.into_iter() {
        new_packages.push(NewPackage::new(package_path)?);
    }

    // And then add all the packages for the server
    let mut added_packages = Vec::with_capacity(new_packages.len());
    for new_package in new_packages.into_iter() {
        added_packages.push(AddedPackage::new(
            new_package,
            dist_utils::path::torrent_data_dir(),
            dist_utils::path::torrent_file_dir(),
            &announce_url,
        )?);
    }

    // And to the database
    for added_package in added_packages.into_iter() {
        package_db.add_package(added_package)?;
    }

    // TODO: seed the torrent and add it to the tracker

    Ok(())
}
