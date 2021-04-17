use anyhow::Result;
use log::{debug, info};
use sysinfo::{ProcessExt, Signal, System, SystemExt};

use std::process::Command;

use crate::cli::ComponentListing;

pub struct ComponentManager {
    components: Vec<Box<dyn Component>>,
}

impl ComponentManager {
    pub fn start(&self) -> Result<()> {
        for component in &self.components {
            component.start()?;
        }

        Ok(())
    }

    pub fn stop(&self) -> Result<()> {
        for component in &self.components {
            component.stop()?
        }

        Ok(())
    }
}

impl From<ComponentListing> for ComponentManager {
    fn from(listing: ComponentListing) -> Self {
        let mut components: Vec<Box<dyn Component>> = Vec::new();

        if !listing.no_database {
            components.push(Box::new(Database));
        }
        if !listing.no_seeder {
            components.push(Box::new(Seeder));
        }
        if !listing.no_tracker {
            components.push(Box::new(Tracker));
        }

        Self { components }
    }
}

pub trait Component {
    fn start(&self) -> Result<()>;

    fn stop(&self) -> Result<()>;
}

pub struct Database;

impl Component for Database {
    fn start(&self) -> Result<()> {
        info!("Starting database server");

        Command::new("named-file-server")
            .arg(&dist_utils::package_db_file())
            .spawn()?;

        Ok(())
    }

    fn stop(&self) -> Result<()> {
        info!("Shutting down database server");

        let mut system = System::new();
        system.refresh_all();
        // Name is truncated here and I don't feel like trying to snag it from the command path
        let processes = system.get_process_by_name("named-file-serv");

        for process in processes {
            debug!("Shutting down PID: {}", process.pid());
            process.kill(Signal::Interrupt);
        }

        Ok(())
    }
}

pub struct Seeder;

impl Component for Seeder {
    fn start(&self) -> Result<()> {
        info!("Starting seeder server");

        todo!()
    }

    fn stop(&self) -> Result<()> {
        info!("Shutting down seeder server");

        todo!()
    }
}

pub struct Tracker;

impl Component for Tracker {
    fn start(&self) -> Result<()> {
        info!("Starting tracker server");

        todo!()
    }

    fn stop(&self) -> Result<()> {
        info!("Shutting down tracker server");

        todo!()
    }
}
