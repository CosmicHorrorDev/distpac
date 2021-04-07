use crate::database::schema::packages;

#[derive(Insertable, Queryable, Debug)]
#[table_name = "packages"]
pub(crate) struct Package {
    pub(crate) name: String,
    pub(crate) version: i32,
    pub(crate) magnet: String,
}
