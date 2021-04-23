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

    // TODO: Would be nice to combine this with `.add_package()`
    pub fn add_package_entry(&self, package: PackageEntry) -> QueryResult<RowID> {
        diesel::insert_into(packages::table)
            .values(&DbPackageEntry::from(package))
            .execute(&self.connection)
    }

    // TODO: this seems specific for no reason. Would be nice to generalize
    pub fn remove_by_name(&self, name: &str) -> QueryResult<RowID> {
        diesel::delete(packages::table.filter(packages::name.eq(name))).execute(&self.connection)
    }

    pub fn list_all(&self) -> QueryResult<Vec<PackageEntry>> {
        let db_packages: Vec<DbPackageEntry> = packages::table.load(&self.connection)?;
        let packages = db_packages.into_iter().map(PackageEntry::from).collect();
        Ok(packages)
    }

    // TODO: this could return multiple packages with different versions. Really should sort by the
    // version number and return the first result.
    pub fn query(&self, name: &str) -> QueryResult<Option<PackageEntry>> {
        let db_packages: Vec<DbPackageEntry> = packages::table
            .filter(packages::name.eq(name))
            .load(&self.connection)?;
        let mut packages: Vec<_> = db_packages.into_iter().map(PackageEntry::from).collect();
        let maybe_package = packages.drain(..).next();

        Ok(maybe_package)
    }
}
