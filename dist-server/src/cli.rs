use clap::Clap;

use std::path::PathBuf;

/// Basic program for managing the distpac server. This involves running a tracker, a dedicated
/// seeder, and an HTTP server for serving the package database.
#[derive(Clap, Debug)]
pub struct Opts {
    #[clap(subcommand)]
    pub subcmd: SubCommand,
}

#[derive(Clap, Debug)]
pub enum SubCommand {
    /// Start all (or just some) of the server components.
    Start(ComponentListing),
    /// Stop all (or just some) of the server components.
    Stop(ComponentListing),
    /// Adds a new package to be served by the distpac.
    Add(AddPackage),
}

#[derive(Clap, Debug, PartialEq)]
pub struct ComponentListing {
    /// Ignore the database component.
    #[clap(short, long)]
    pub no_database: bool,
    /// Ignore the seeder component.
    #[clap(short, long)]
    pub no_seeder: bool,
    /// Ignore the tracker component.
    #[clap(short, long)]
    pub no_tracker: bool,
}

#[derive(Clap, Debug)]
pub struct AddPackage {
    /// Paths to all the packages to add.
    pub packages: Vec<PathBuf>,
}
