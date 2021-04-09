mod cli;

use clap::Clap;

use crate::cli::{Opts, SubCommand};

fn main() {
    let Opts { subcmd } = Opts::parse();

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
