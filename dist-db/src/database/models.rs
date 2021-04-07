use crate::{database::schema::packages, models::Package};

#[derive(Insertable, Queryable, Debug)]
#[table_name = "packages"]
pub(crate) struct DbPackage {
    pub(crate) name: String,
    pub(crate) version: i32,
    pub(crate) magnet: String,
}

impl From<Package> for DbPackage {
    fn from(package: Package) -> Self {
        let Package {
            name,
            version,
            magnet,
        } = package;

        Self {
            name,
            version: version.as_i32(),
            magnet,
        }
    }
}
