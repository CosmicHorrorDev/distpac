use dist_package::manifest::Version;

use crate::database::models::DbPackageEntry;

#[derive(Debug)]
pub struct PackageEntry {
    pub name: String,
    pub version: Version,
    pub magnet: String,
}

impl PackageEntry {
    fn new(name: String, version: Version, magnet: String) -> Self {
        Self {
            name,
            version,
            magnet,
        }
    }
}

impl From<DbPackageEntry> for PackageEntry {
    fn from(db_package: DbPackageEntry) -> Self {
        let version = Version::from(db_package.version);
        Self::new(db_package.name, version, db_package.magnet)
    }
}
