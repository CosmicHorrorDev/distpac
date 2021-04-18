use anyhow::Result;
use log::info;
use transmission_wrapper::{Transmission, TransmissionOpts};

use std::process::Command;

use crate::cli::ComponentListing;

const DATABASE_SERVER_NAME: &str = "named-file-server";

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

    pub fn stop(&self) {
        for component in &self.components {
            component.stop();
        }
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

    fn stop(&self);
}

pub struct Database;

impl Component for Database {
    fn start(&self) -> Result<()> {
        info!("Starting database server");

        Command::new(DATABASE_SERVER_NAME)
            .arg(&dist_utils::path::package_db_file())
            .spawn()?;

        Ok(())
    }

    fn stop(&self) {
        info!("Shutting down database server");
        dist_utils::misc::stop_process_by_name(DATABASE_SERVER_NAME);
    }
}

pub struct Seeder;

impl Component for Seeder {
    fn start(&self) -> Result<()> {
        info!("Starting seeder server");
        let opts = TransmissionOpts::new().download_dir(dist_utils::path::torrent_data_dir());
        Transmission::start(&opts)?;

        Ok(())
    }

    fn stop(&self) {
        info!("Shutting down seeder server");
        // This will only stop the server if it was already running
        if let Some(daemon) = Transmission::from_running() {
            daemon.stop();
        }
    }
}

pub struct Tracker;

impl Component for Tracker {
    fn start(&self) -> Result<()> {
        info!("Starting tracker server");

        todo!()
    }

    fn stop(&self) {
        info!("Shutting down tracker server");

        todo!()
    }
}
