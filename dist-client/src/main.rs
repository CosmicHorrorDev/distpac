mod cli;
mod config;

use anyhow::Result;
use clap::Clap;
use log::{debug, info};

use std::{fs::File, io};

use crate::{
    cli::{Opts, Packages, SubCommand},
    config::Config,
};

fn main() -> Result<()> {
    let Opts {
        quiet,
        verbose,
        subcmd,
    } = Opts::parse();

    stderrlog::new()
        .module(module_path!())
        .quiet(quiet)
        .verbosity(verbose)
        .init()?;
    debug!("Subcommand: {:#?}", subcmd);

    let config_path = dist_utils::client_config_path();
    let config_file = std::fs::File::open(&config_path)?;
    let config: Config = serde_yaml::from_reader(config_file)?;
    debug!("Config: {:#?}", config);

    match subcmd {
        SubCommand::Sync => {
            // Get the latest package database
            info!("Attempting to sync the latest package database...");
            let response = ureq::get(&format!("{}/packages.db", config.server_url)).call()?;
            let mut db_file = File::create(&dist_utils::package_db_file())?;
            let mut response_content = response.into_reader();

            info!("Saving the file locally...");
            io::copy(&mut response_content, &mut db_file)?;
            info!("Finished syncing");
        }
        SubCommand::Install(Packages { packages }) => {
            todo!()
        }
        SubCommand::Remove(Packages { packages }) => {
            todo!()
        }
        SubCommand::List(list_opts) => {
            todo!()
        }
        SubCommand::Search(search_query) => {
            todo!()
        }
    }

    Ok(())
}
