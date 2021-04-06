pub mod models;
mod schema;

use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use dotenv::dotenv;

use std::env;

use crate::{
    database::{models::Package, schema::packages},
    models::Version,
};

pub struct DistpacDB {
    connection: SqliteConnection,
}

impl DistpacDB {
    // TODO: move away from dotenv to using `dirs` instead
    // TODO: actually do proper error handling here
    pub fn new() -> Self {
        dotenv().ok();

        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let connection = SqliteConnection::establish(&database_url)
            .expect(&format!("Error connecting to {}", database_url));

        Self { connection }
    }

    pub fn add_package(&self, version: Version, name: &str, magnet: &str) {
        let new_package = Package {
            version: version.as_i32(),
            name: name.to_string(),
            magnet: magnet.to_string(),
        };

        diesel::insert_into(packages::table)
            .values(&new_package)
            .execute(&self.connection)
            .expect("Failed inserting package into db");
    }
}
