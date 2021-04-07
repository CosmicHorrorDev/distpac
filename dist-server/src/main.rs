use clap::Clap;

use crate::{
    cli::{Opts, SubCommand},
    components::ComponentManager,
};

mod cli;
mod components;

fn main() {
    let Opts { subcmd } = Opts::parse();

    match subcmd {
        SubCommand::Start(component_listing) => {
            ComponentManager::from(component_listing).start();
        }
        SubCommand::Stop(component_listing) => {
            ComponentManager::from(component_listing).stop();
        }
        SubCommand::Add(add_package) => {
            todo!()
        }
    }
}
