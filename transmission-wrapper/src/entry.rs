use getset::{Getters, Setters};

use std::{process::Command, str::FromStr};

use crate::{bytes::Bytes, constants::REMOTE_NAME, error::Error};

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Status {
    Downloading,
    Idle,
    Seeding,
    Stopped,
    UpAndDown,
    Verifying,
}

impl FromStr for Status {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Downloading" => Ok(Self::Downloading),
            "Idle" => Ok(Self::Idle),
            "Seeding" => Ok(Self::Seeding),
            "Stopped" => Ok(Self::Stopped),
            "Up & Down" => Ok(Self::UpAndDown),
            "Verifying" => Ok(Self::Verifying),
            _ => Err(Self::Err::InvalidEntryFormat),
        }
    }
}

#[derive(Getters, Setters, Clone, Debug, PartialEq)]
#[getset(get = "pub")]
pub struct Entry {
    id: u64,
    size: Bytes,
    downloaded: Bytes,
    status: Status,
    name: String,
}

impl Entry {
    pub fn from_id(id: u64) -> Result<Self, Error> {
        let output = Command::new(REMOTE_NAME)
            .arg("--torrent")
            .arg(id.to_string())
            .arg("--info")
            .output()?;

        String::from_utf8(output.stdout)?.parse()
    }

    pub fn completed(id: u64, downloaded: Bytes, status: Status, name: String) -> Self {
        Self::new(id, downloaded, downloaded, status, name)
    }

    pub fn is_finished(&self) -> bool {
        self.size != Bytes(0.0) && self.size == self.downloaded
    }

    pub fn new(id: u64, size: Bytes, downloaded: Bytes, status: Status, name: String) -> Self {
        Self {
            id,
            size,
            downloaded,
            status,
            name,
        }
    }

    pub fn update(&mut self, downloaded: Bytes, status: Status) {
        self.downloaded = downloaded;
        self.status = status;
    }
}

impl FromStr for Entry {
    type Err = Error;

    // Parses and entry from the `transmission-remote --torrent <torrent id> --info` output
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut info = Vec::new();

        // Go over each line of the output picking out special info. A trimmed version is below
        // NAME
        //     Id: <torrent id>
        //     Name: <torrent name>
        //
        // TRANSFER
        //     State: <torrent status>
        //     Have: <downloaded> (unwanted junk)
        //     Total size: <torrent size> (unwanted junk)
        for line in s.lines() {
            let line = line.trim();

            if let Some(id) = line.strip_prefix("Id: ") {
                info.push(id);
            } else if let Some(size_str) = line.strip_prefix("Total size: ") {
                // Strip out the unwanted junk
                let size = size_str
                    .split('(')
                    .next()
                    .ok_or(Self::Err::InvalidEntryFormat)?
                    .trim();
                info.push(size);
            } else if let Some(downloaded_str) = line.strip_prefix("Have: ") {
                // Strip out the unwanted junk
                let downloaded = downloaded_str
                    .split('(')
                    .next()
                    .ok_or(Self::Err::InvalidEntryFormat)?
                    .trim();
                info.push(downloaded);
            } else if let Some(status_str) = line.strip_prefix("State: ") {
                // Status can have some extra junk in parentheses when it's verifying
                let status = status_str
                    .split('(')
                    .next()
                    .ok_or(Self::Err::InvalidEntryFormat)?
                    .trim();
                info.push(status);
            } else if let Some(name) = line.strip_prefix("Name: ") {
                info.push(name);
            }
        }

        match info.as_slice() {
            [id_str, name_str, status, downloaded_str, size_str] => Ok(Self {
                id: id_str.parse().map_err(|_| Self::Err::InvalidEntryFormat)?,
                size: if *size_str == "None" {
                    Bytes(0.0)
                } else {
                    size_str.parse()?
                },
                downloaded: if *downloaded_str == "None" {
                    Bytes(0.0)
                } else {
                    downloaded_str.parse()?
                },
                status: status.parse()?,
                name: name_str.to_string(),
            }),
            _ => Err(Self::Err::InvalidEntryFormat),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::{fs, path::Path};

    type BoxResult<T> = Result<T, Box<dyn std::error::Error>>;

    #[test]
    fn parse_entry() -> BoxResult<()> {
        let sample_file = Path::new("tests").join("corpus").join("torrent_info.txt");
        let torrent_info = fs::read_to_string(&sample_file)?;
        let entry: Entry = torrent_info.parse()?;

        let size = Bytes::from(786.8 * 1_000_000.0);
        assert_eq!(
            entry,
            Entry::completed(
                1,
                size,
                Status::Idle,
                "archlinux-2021.04.01-x86_64.iso".to_owned()
            )
        );

        Ok(())
    }
}
