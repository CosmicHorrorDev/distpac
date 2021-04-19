use dist_package::manifest::Version;
use getset::Getters;

use crate::database::models::DbPackageEntry;

#[derive(Getters, Debug)]
#[getset(get = "pub")]
pub struct PackageEntry {
    pub(crate) torrent_name: String,
    pub(crate) name: String,
    pub(crate) version: Version,
    pub(crate) magnet: String,
    pub(crate) size: u64,
}

impl PackageEntry {
    fn new(name: String, version: Version, magnet: String, size: u64) -> Self {
        Self {
            torrent_name: format!("{}-{}", name, version),
            name,
            version,
            magnet,
            size,
        }
    }
}

impl From<DbPackageEntry> for PackageEntry {
    fn from(db_package: DbPackageEntry) -> Self {
        let version = Version::from(db_package.version);
        Self::new(
            db_package.name,
            version,
            db_package.magnet,
            db_package.size_bytes as u64,
        )
    }
}
