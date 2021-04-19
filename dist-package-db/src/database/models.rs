use dist_package::{AddedPackage, Torrent};

use crate::{database::schema::packages, models::PackageEntry};

#[derive(Insertable, Queryable, Debug)]
#[table_name = "packages"]
pub(crate) struct DbPackageEntry {
    pub(crate) torrent_name: String,
    pub(crate) name: String,
    pub(crate) version: i32,
    pub(crate) magnet: String,
    pub(crate) size_bytes: i32,
}

impl DbPackageEntry {
    pub fn new(
        torrent_name: String,
        name: String,
        version: i32,
        magnet: String,
        size_bytes: i32,
    ) -> Self {
        Self {
            torrent_name,
            name,
            version,
            magnet,
            size_bytes,
        }
    }
}

impl From<PackageEntry> for DbPackageEntry {
    fn from(package_entry: PackageEntry) -> Self {
        let PackageEntry {
            torrent_name,
            name,
            version,
            magnet,
            size,
        } = package_entry;

        Self::new(torrent_name, name, version.as_i32(), magnet, size as i32)
    }
}

impl From<AddedPackage> for DbPackageEntry {
    fn from(package: AddedPackage) -> Self {
        let AddedPackage {
            name,
            version,
            torrent:
                Torrent {
                    name: torrent_name,
                    magnet,
                    size,
                    ..
                },
            ..
        } = package;

        Self::new(torrent_name, name, version.as_i32(), magnet, size as i32)
    }
}
