pub mod models;
mod schema;

use diesel::prelude::*;
use diesel::result::{ConnectionResult, QueryResult};
use diesel::sqlite::SqliteConnection;

use crate::{
    database::{models::Package, schema::packages},
    models::Version,
};

pub struct DistpacDB {
    connection: SqliteConnection,
}

impl DistpacDB {
    pub fn new() -> ConnectionResult<Self> {
        // Get the database connection url
        let data_dir = dirs_next::data_dir().expect("Error handling is for losers");
        let package_db = data_dir.join("package_list.db");
        let database_url = package_db.to_str().unwrap();

        let connection = SqliteConnection::establish(&database_url)?;
        Ok(Self { connection })
    }

    // TODO: is the usize here a rowid?
    pub fn add_package(&self, version: Version, name: &str, magnet: &str) -> QueryResult<usize> {
        let new_package = Package {
            version: version.as_i32(),
            name: name.to_string(),
            magnet: magnet.to_string(),
        };

        diesel::insert_into(packages::table)
            .values(&new_package)
            .execute(&self.connection)
    }
}
