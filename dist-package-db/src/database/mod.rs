pub mod models;
mod schema;

use diesel::prelude::*;
use diesel::result::QueryResult;
use diesel::sqlite::SqliteConnection;
use dist_package::AddedPackage;

use std::{fs, path::Path};

use crate::{
    database::{models::DbPackageEntry, schema::packages},
    error::DatabaseError,
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
    pub fn connect(db_path: &Path, action: MissingDBAction) -> Result<Self, DatabaseError> {
        let database_url = db_path.to_str().ok_or(DatabaseError::InvalidDatabaseUrl)?;

        let connection = if db_path.exists() {
            SqliteConnection::establish(&database_url)?
        } else {
            // Diesel will create a new SQLite DB when connecting, so need to decide what to do if
            // it's missing
            match action {
                MissingDBAction::Create => {
                    // Create any needed directories
                    fs::create_dir_all(
                        &db_path
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

    pub fn add_package(&self, package: AddedPackage) -> QueryResult<RowID> {
        diesel::insert_into(packages::table)
            .values(&DbPackageEntry::from(package))
            .execute(&self.connection)
    }
}
