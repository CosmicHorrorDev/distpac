use clap::Clap;

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
    /// Install the listed package.
    Install(Package),
    /// Remove the installed package
    Remove(Package),
    /// Operations related to listing packages.
    List(ListOpts),
}

#[derive(Clap, Debug)]
pub struct Package {
    /// Package to operate on.
    pub name: String,
}

#[derive(Clap, Debug)]
pub struct ListOpts {
    /// List only installed packages instead of all available.
    #[clap(long)]
    pub installed: bool,
}
