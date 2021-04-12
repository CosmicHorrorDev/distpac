use anyhow::Result;
use clap::Clap;
use dist_package::{AddedPackage, NewPackage, PackageOpts};
use dist_package_db::database::{DistpacDB, MissingDBAction};

use crate::{
    cli::{AddPackage, Opts, SubCommand},
    components::ComponentManager,
};

mod cli;
mod components;

fn main() -> Result<()> {
    let Opts { subcmd } = Opts::parse();

    // Setup all the common directories
    dist_utils::create_dirs()?;

    match subcmd {
        SubCommand::Start(component_listing) => {
            ComponentManager::from(component_listing).start();
        }
        SubCommand::Stop(component_listing) => {
            ComponentManager::from(component_listing).stop();
        }
        SubCommand::Add(AddPackage { package_paths }) => {
            let opts = PackageOpts::new()
                .packages_dir(dist_utils::torrent_data_dir())
                .torrent_dir(dist_utils::torrent_file_dir());
            let package_db =
                DistpacDB::connect(&dist_utils::package_db_file(), MissingDBAction::Create)?;

            // Validate all the new packages first
            let mut new_packages = Vec::with_capacity(package_paths.len());
            for package_path in package_paths.into_iter() {
                new_packages.push(NewPackage::new(package_path)?);
            }

            // And then add all the packages for the server
            let mut added_packages = Vec::with_capacity(new_packages.len());
            for new_package in new_packages.into_iter() {
                added_packages.push(AddedPackage::new_with_opts(new_package, &opts)?);
            }

            // And to the database
            for added_package in added_packages.into_iter() {
                package_db.add_package(added_package)?;
            }

            // TODO: seed the torrent and add it to the tracker
        }
    }

    Ok(())
}
