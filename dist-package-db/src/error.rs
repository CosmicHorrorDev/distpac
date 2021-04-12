use diesel::result::ConnectionError;
use diesel_migrations::RunMigrationsError;
use thiserror::Error;

use std::io;

#[derive(Error, Debug)]
pub enum DatabaseError {
    #[error("Failed establishing a connection to the database Error: {0}")]
    Connection(#[from] ConnectionError),
    #[error("Error setting up database URL")]
    InvalidDatabaseUrl,
    #[error("Failed running a database migration Error: {0}")]
    Migration(#[from] RunMigrationsError),
    #[error("The database is missing and is configured to not be created automatically")]
    MissingDatabase,
    #[error("Encountered IO error when dealing with database Error: {0}")]
    Io(#[from] io::Error),
}
