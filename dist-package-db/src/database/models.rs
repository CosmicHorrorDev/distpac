use crate::{database::schema::packages, models::PackageEntry};

#[derive(Insertable, Queryable, Debug)]
#[table_name = "packages"]
pub(crate) struct DbPackageEntry {
    pub(crate) name: String,
    pub(crate) version: i32,
    pub(crate) magnet: String,
}

impl From<PackageEntry> for DbPackageEntry {
    fn from(package_entry: PackageEntry) -> Self {
        let PackageEntry {
            name,
            version,
            magnet,
            ..
        } = package_entry;

        Self {
            name,
            version: version.as_i32(),
            magnet,
        }
    }
}
