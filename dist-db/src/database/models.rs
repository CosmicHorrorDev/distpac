use crate::database::schema::packages;

#[derive(Insertable, Queryable, Debug)]
#[table_name = "packages"]
pub(crate) struct DbPackage {
    pub(crate) name: String,
    pub(crate) version: i32,
    pub(crate) magnet: String,
}
