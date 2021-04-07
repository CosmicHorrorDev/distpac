pub mod models;
mod schema;

use diesel::prelude::*;
use diesel::result::{ConnectionResult, QueryResult};
use diesel::sqlite::SqliteConnection;

use crate::{
    database::{models::DbPackageEntry, schema::packages},
    models::PackageEntry,
};

pub type RowID = usize;

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

    pub fn add_package(&self, package: PackageEntry) -> QueryResult<RowID> {
        diesel::insert_into(packages::table)
            .values(&DbPackageEntry::from(package))
            .execute(&self.connection)
    }
}
