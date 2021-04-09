#[macro_use]
extern crate diesel;

use diesel::prelude::*;
use diesel::result::{ConnectionResult, QueryResult};
use diesel::sqlite::SqliteConnection;

use std::path::Path;

fn main() {
    let database_url = "/home/lovecraft/.data/distpac/databases/installed.db";
    let connection = SqliteConnection::establish(database_url);
    println!("Hello, world!");
}
