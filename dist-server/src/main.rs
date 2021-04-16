use anyhow::Result;
use clap::Clap;

use crate::{
    cli::{AddPackage, Opts, SubCommand},
    components::ComponentManager,
    packages::add_packages,
};

mod cli;
mod components;
mod packages;

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
            add_packages(package_paths)?;
        }
    }

    Ok(())
}
