#[macro_use]
extern crate diesel;
extern crate dotenv;

mod database;
mod models;
mod torrent;

use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use dotenv::dotenv;

use std::{env, path::Path};

use crate::{database::models::Package, torrent::Torrent};

pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

pub fn add_package(conn: &SqliteConnection, version: &str, name: &str, magnet: &str) {
    use database::schema::packages;

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

fn main() {
    let torrent = Torrent::create(Path::new("/home/lovecraft/Downloads"));
    println!("{:#?}", torrent);
    // use crate::database::schema::packages::dsl::*;

    // let connection = establish_connection();
    // // add_package(&connection, "0.1.0", "chrome-stable", "magnet://chrome");
    // let results = packages
    //     .filter(name.eq("chrome-stable"))
    //     .load::<Package>(&connection)
    //     .expect("Error querying packages");

    // println!("Displaying {} packages", results.len());
    // for package in results {
    //     println!("{:#?}", crate::models::Package::from(package));
    // }
}
