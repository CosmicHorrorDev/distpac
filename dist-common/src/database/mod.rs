pub mod models;
pub mod schema;

use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use dotenv::dotenv;

use std::env;

use crate::database::models::Package;

pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

pub fn add_package(conn: &SqliteConnection, version: &str, name: &str, magnet: &str) {
    use crate::database::schema::packages;

    let new_package = Package {
        version: version.to_string(),
        name: name.to_string(),
        magnet: magnet.to_string(),
    };

    diesel::insert_into(packages::table)
        .values(&new_package)
        .execute(conn)
        .expect("Failed inserting package into db");
}
