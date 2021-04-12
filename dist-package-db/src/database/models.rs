use dist_package::{manifest::Manifest, AddedPackage, Torrent};

use crate::{
    database::schema::packages,
    models::{PackageEntry, Version},
};

#[derive(Insertable, Queryable, Debug)]
#[table_name = "packages"]
pub(crate) struct DbPackageEntry {
    pub(crate) name: String,
    pub(crate) version: i32,
    pub(crate) magnet: String,
}

impl DbPackageEntry {
    pub fn new(name: String, version: i32, magnet: String) -> Self {
        Self {
            name,
            version,
            magnet,
        }
    }
}

impl From<PackageEntry> for DbPackageEntry {
    fn from(package_entry: PackageEntry) -> Self {
        let PackageEntry {
            name,
            version,
            magnet,
        } = package_entry;

        Self {
            name,
            version: version.as_i32(),
            magnet,
        }
    }
}

impl From<AddedPackage> for DbPackageEntry {
    fn from(package: AddedPackage) -> Self {
        let AddedPackage {
            manifest: Manifest { name, version },
            torrent: Torrent { magnet, .. },
            ..
        } = package;

        Self::new(name, version.parse::<Version>().unwrap().as_i32(), magnet)
    }
}
