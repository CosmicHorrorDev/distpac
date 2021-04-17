use serde::{de::Error, Deserialize, Deserializer};
use serde_yaml;

use std::{cmp::Ordering, convert::TryFrom, fmt, fs::File, path::Path, str::FromStr};

use crate::error::{PackageError, ParseVersionError};

#[derive(Deserialize, Debug)]
pub struct Manifest {
    pub name: String,
    pub version: Version,
}

impl TryFrom<&Path> for Manifest {
    type Error = PackageError;

    fn try_from(manifest_path: &Path) -> Result<Self, Self::Error> {
        let file = File::open(manifest_path)?;
        let manifest = serde_yaml::from_reader(file)?;

        Ok(manifest)
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Version(pub u8, pub u8, pub u8);

impl Version {
    pub fn new(major: u8, minor: u8, patch: u8) -> Self {
        Self(major, minor, patch)
    }

    pub fn major(&self) -> u8 {
        self.0
    }

    pub fn minor(&self) -> u8 {
        self.1
    }

    pub fn patch(&self) -> u8 {
        self.2
    }

    pub fn as_i32(&self) -> i32 {
        // Pack the 4 bytes so that it's [empty][major][minor][patch]
        // This is done due to limitations on what types can be used as INTEGER for SQLite
        // https://github.com/diesel-rs/diesel/issues/852
        // Conversely the reverse is implemented with `From<i32>`
        let mut packed = 0i32;
        packed += (self.major() as i32) << 16;
        packed += (self.minor() as i32) << 8;
        packed += self.patch() as i32;

        packed
    }
}

impl fmt::Display for Version {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}.{}.{}", self.major(), self.minor(), self.patch())
    }
}

impl PartialOrd for Version {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Version {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.major().cmp(&other.major()) {
            Ordering::Equal => match self.minor().cmp(&other.minor()) {
                Ordering::Equal => self.patch().cmp(&other.patch()),
                order => order,
            },
            order => order,
        }
    }
}

impl From<i32> for Version {
    fn from(packed: i32) -> Self {
        let extract_byte = |value, shift| ((value >> shift) & 0xFF) as u8;

        let major = extract_byte(packed, 16);
        let minor = extract_byte(packed, 8);
        let patch = extract_byte(packed, 0);

        Self::new(major, minor, patch)
    }
}

impl FromStr for Version {
    type Err = ParseVersionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<_> = s.split('.').collect();

        // There should be three values
        if parts.len() != 3 {
            return Err(Self::Err::InvalidValuesCount(parts.len()));
        }

        // Now parse each of the three version segments
        let parse_u64 = |s: &str| -> Result<u8, Self::Err> {
            s.parse()
                .map_err(|_| Self::Err::InvalidValue(s.to_string()))
        };

        let major = parse_u64(parts[0])?;
        let minor = parse_u64(parts[1])?;
        let patch = parse_u64(parts[2])?;

        Ok(Self::new(major, minor, patch))
    }
}

impl<'de> Deserialize<'de> for Version {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(D::Error::custom)
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
