use anyhow::{anyhow, Error as AnyhowError, Result as AnyhowResult};

use std::{cmp::Ordering, fmt, str::FromStr};

use crate::database::models::Package as DbPackage;

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
    type Err = AnyhowError;

    fn from_str(s: &str) -> AnyhowResult<Self> {
        let mut parts = s.split('.');

        let parse_u64 = |maybe_str: Option<&str>| -> AnyhowResult<u64> {
            match maybe_str {
                Some(s) => {
                    let val = s.parse()?;
                    Ok(val)
                }
                None => Err(anyhow!("Version missing value segment")),
            }
        };
        let major = parse_u64(parts.next())?;
        let minor = parse_u64(parts.next())?;
        let patch = parse_u64(parts.next())?;

        Ok(Self::new(major, minor, patch))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn version_from_str() -> AnyhowResult<()> {
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
    fn version_display() -> AnyhowResult<()> {
        let version: Version = "1.2.3".parse()?;
        assert_eq!(format!("{}", version), "1.2.3");

        Ok(())
    }
}
