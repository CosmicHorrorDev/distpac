mod cli;

use anyhow::Result;
use clap::Clap;
use log::debug;

use crate::cli::{Opts, SubCommand};

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

    match subcmd {
        SubCommand::Sync => {
            todo!()
        }
        SubCommand::Install(packages) => {
            todo!()
        }
        SubCommand::Remove(packages) => {
            todo!()
        }
        SubCommand::List(list_opts) => {
            todo!()
        }
        SubCommand::Search(search_query) => {
            todo!()
        }
    }
}
