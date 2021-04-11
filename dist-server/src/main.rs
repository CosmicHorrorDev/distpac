use anyhow::Result;
use clap::Clap;
use dist_package::NewPackage;

use crate::{
    cli::{AddPackage, Opts, SubCommand},
    components::ComponentManager,
};

mod cli;
mod components;

fn main() -> Result<()> {
    let Opts { subcmd } = Opts::parse();

    match subcmd {
        SubCommand::Start(component_listing) => {
            ComponentManager::from(component_listing).start();
        }
        SubCommand::Stop(component_listing) => {
            ComponentManager::from(component_listing).stop();
        }
        SubCommand::Add(AddPackage { package_paths }) => {
            let mut new_packages = Vec::new();
            // Validate all the packages first
            for package_path in package_paths {
                new_packages.push(NewPackage::new(package_path)?);
            }

            todo!();
        }
    }

    Ok(())
}
