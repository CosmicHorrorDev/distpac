pub mod models;
mod schema;

use diesel::prelude::*;
use diesel::result::QueryResult;
use diesel::sqlite::SqliteConnection;

use std::fs;

use crate::{
    database::{models::DbPackageEntry, schema::packages},
    error::DatabaseError,
    models::PackageEntry,
};

embed_migrations!("./migrations");

pub type RowID = usize;

#[derive(Debug)]
pub enum MissingDBAction {
    Create,
    RaiseError,
}

impl Default for MissingDBAction {
    fn default() -> Self {
        Self::RaiseError
    }
}

pub struct DistpacDB {
    connection: SqliteConnection,
}

impl DistpacDB {
    pub fn connect(action: MissingDBAction) -> Result<Self, DatabaseError> {
        // Get the database connection url
        let data_dir = dirs_next::data_dir().expect("Error handling is for losers");
        let package_db = data_dir
            .join("distpac")
            .join("databases")
            .join("package_list.db");
        let database_url = package_db
            .to_str()
            .ok_or(DatabaseError::InvalidDatabaseUrl)?;

        let connection = if package_db.exists() {
            SqliteConnection::establish(&database_url)?
        } else {
            // Diesel will create a new SQLite DB when connecting, so need to decide what to do if
            // it's missing
            match action {
                MissingDBAction::Create => {
                    // Create any needed directories
                    fs::create_dir_all(
                        &package_db
                            .parent()
                            .expect("Package database path must have a parent"),
                    )?;

                    // And create the database
                    let connection = SqliteConnection::establish(&database_url)?;
                    embedded_migrations::run(&connection)?;
                    connection
                }
                MissingDBAction::RaiseError => {
                    return Err(DatabaseError::MissingDatabase);
                }
            }
        };

        Ok(Self { connection })
    }

    pub fn add_package(&self, package: PackageEntry) -> QueryResult<RowID> {
        diesel::insert_into(packages::table)
            .values(&DbPackageEntry::from(package))
            .execute(&self.connection)
    }
}
