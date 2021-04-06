use std::{cmp::Ordering, fmt, str::FromStr};

use crate::database::models::Package as DbPackage;
use crate::errors::ParseVersionError;

#[derive(Debug)]
pub struct Package {
    pub name: String,
    pub version: Version,
    pub magnet: String,
}

impl Package {
    fn new(name: String, version: Version, magnet: String) -> Self {
        Self {
            name,
            version,
            magnet,
        }
    }
}

impl From<DbPackage> for Package {
    fn from(db_package: DbPackage) -> Self {
        let version: Version = db_package.version.parse().unwrap();
        Self::new(db_package.name, version, db_package.magnet)
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Version {
    pub major: u64,
    pub minor: u64,
    pub patch: u64,
}

impl Version {
    pub fn new(major: u64, minor: u64, patch: u64) -> Self {
        Self {
            major,
            minor,
            patch,
        }
    }
}

impl fmt::Display for Version {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}.{}.{}", self.major, self.minor, self.patch)
    }
}

impl PartialOrd for Version {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Version {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.major.cmp(&other.major) {
            Ordering::Equal => match self.minor.cmp(&other.minor) {
                Ordering::Equal => self.patch.cmp(&other.patch),
                order => order,
            },
            order => order,
        }
    }
}

impl FromStr for Version {
    type Err = ParseVersionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<_> = s.split('.').collect();

        // There should be three values
        if parts.len() != 3 {
            return Err(Self::Err::invalid_count(parts.len()));
        }

        // Now parse each of the three version segments
        let parse_u64 = |s: &str| -> Result<u64, Self::Err> {
            s.parse()
                .map_err(|_| Self::Err::invalid_value(s.to_string()))
        };

        let major = parse_u64(parts[0])?;
        let minor = parse_u64(parts[1])?;
        let patch = parse_u64(parts[2])?;

        Ok(Self::new(major, minor, patch))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    type TestResult<T> = Result<T, Box<dyn std::error::Error>>;

    #[test]
    fn version_from_str() -> TestResult<()> {
        let good: Version = "1.2.3".parse()?;
        assert_eq!(good, Version::new(1, 2, 3));

        let bad = "1.2.".parse::<Version>();
        assert!(bad.is_err());

        let bad = "asdf".parse::<Version>();
        assert!(bad.is_err());

        Ok(())
    }

    #[test]
    fn version_ord() {
        let one_two_three = Version::new(1, 2, 3);
        let one_two_two = Version::new(1, 2, 2);
        let two_zero_zero = Version::new(2, 0, 0);

        assert!(one_two_three > one_two_two);
        assert!(one_two_three >= one_two_two);
        assert!(one_two_three == one_two_three);
        assert!(one_two_three < two_zero_zero);
    }

    #[test]
    fn version_display() -> TestResult<()> {
        let version: Version = "1.2.3".parse()?;
        assert_eq!(format!("{}", version), "1.2.3");

        Ok(())
    }
}
