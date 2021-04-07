use crate::cli::ComponentListing;

pub struct ComponentManager {
    components: Vec<Box<dyn Component>>,
}

impl ComponentManager {
    pub fn start(&self) {
        for component in &self.components {
            component.start();
        }
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
    fn start(&self);

    fn stop(&self);
}

pub struct Database;

impl Component for Database {
    fn start(&self) {
        todo!();
    }

    fn stop(&self) {
        todo!();
    }
}

pub struct Seeder;

impl Component for Seeder {
    fn start(&self) {
        todo!()
    }

    fn stop(&self) {
        todo!()
    }
}

pub struct Tracker;

impl Component for Tracker {
    fn start(&self) {
        todo!()
    }

    fn stop(&self) {
        todo!()
    }
}
