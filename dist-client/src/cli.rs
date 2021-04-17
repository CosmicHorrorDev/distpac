use clap::Clap;

use std::path::PathBuf;

/// Basic program for managing the distpac client. This includes operations for syncing the package
/// listing from the server, listing, and searching from the package listing, and finally installing
/// and removing packages.
#[derive(Clap, Debug)]
pub struct Opts {
    /// Silence all output
    #[clap(short, long)]
    pub quiet: bool,
    /// Increase verbosity
    #[clap(short, long, parse(from_occurrences))]
    pub verbose: usize,
    #[clap(subcommand)]
    pub subcmd: SubCommand,
}

#[derive(Clap, Debug)]
pub enum SubCommand {
    /// Sync the package listing with the server.
    Sync,
    /// Install the listed packages.
    Install(Packages),
    /// Remove the installed packages
    Remove(Packages),
    /// Operations related to listing packages.
    List(ListOpts),
    /// Search the packages in the local listing.
    Search(SearchQuery),
}

#[derive(Clap, Debug)]
pub struct Packages {
    /// Packages to operate on.
    pub packages: Vec<PathBuf>,
}

#[derive(Clap, Debug)]
pub struct ListOpts {
    /// List only installed packages instead of all available.
    #[clap(long)]
    pub installed: bool,
}

#[derive(Clap, Debug)]
pub struct SearchQuery {
    /// Terms to narrow the package search.
    pub query_terms: Vec<String>,
}
