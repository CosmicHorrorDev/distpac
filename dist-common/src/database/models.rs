use crate::database::schema::packages;

#[derive(Insertable, Queryable, Debug)]
#[table_name = "packages"]
pub struct Package {
    pub name: String,
    pub version: i32,
    pub magnet: String,
}
