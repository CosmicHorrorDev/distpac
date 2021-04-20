use dist_utils::misc::stop_process_by_name;
use sysinfo::{System, SystemExt};

use std::{path::PathBuf, process::Command};

use crate::{
    constants::{DAEMON_NAME, REMOTE_NAME},
    entry::Entry,
    error::Error,
};

mod bytes;
mod constants;
pub mod entry;
pub mod error;

#[derive(Default, Debug)]
pub struct TransmissionOpts {
    pub download_dir: Option<PathBuf>,
}

impl TransmissionOpts {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn download_dir(mut self, download_dir: PathBuf) -> Self {
        self.download_dir = Some(download_dir);
        self
    }
}

// TODO: ideally there should be a global lock so that only one of these can be created at a time
pub struct Transmission {
    entries: Vec<Entry>,
}

impl Transmission {
    fn empty() -> Self {
        Self {
            entries: Vec::new(),
        }
    }

    pub fn start(opts: &TransmissionOpts) -> Result<Self, Error> {
        // Start the daemon if it's not already running
        if !Self::is_running() {
            let mut command = Command::new(DAEMON_NAME);

            if let Some(download_dir) = &opts.download_dir {
                command.arg("--download-dir").arg(&download_dir);
            }

            command.spawn()?;
        }

        Ok(Self::empty())
    }

    pub fn from_running() -> Option<Self> {
        Self::is_running().then(Self::empty)
    }

    pub fn stop(self) {
        stop_process_by_name(DAEMON_NAME);
    }

    fn is_running() -> bool {
        let mut system = System::new();
        system.refresh_all();
        let processes = system.get_process_by_name(DAEMON_NAME);

        !processes.is_empty()
    }

    fn get_mut_by_id(&mut self, id: u64) -> Option<&mut Entry> {
        self.entries.iter_mut().find(|entry| entry.id() == &id)
    }

    pub fn get_by_name(&self, name: &str) -> Option<&Entry> {
        self.entries.iter().find(|entry| entry.name() == name)
    }

    pub fn refresh(&mut self) -> Result<(), Error> {
        let output = Command::new(REMOTE_NAME).arg("--list").output()?;
        let stdout = String::from_utf8(output.stdout)?;

        self.update_entries(&stdout)
    }

    fn update_entries(&mut self, s: &str) -> Result<(), Error> {
        for line in s.lines().skip(1) {
            let line = line.trim();
            if line.starts_with("Sum:") {
                break;
            }

            // Parse info for each entry
            let mut pieces = line.split_whitespace();
            let id = pieces
                .next()
                .ok_or(Error::InvalidEntryFormat)?
                .parse()
                .map_err(|_| Error::InvalidEntryFormat)?;
            let percentage = pieces.next().ok_or(Error::InvalidEntryFormat)?;
            let downloaded = {
                let downloaded_prefix = pieces.next().ok_or(Error::InvalidEntryFormat)?;
                let downloaded_suffix = pieces.next().ok_or(Error::InvalidEntryFormat)?;
                format!("{} {}", downloaded_prefix, downloaded_suffix).parse()?
            };
            let mut pieces = pieces.skip(4);
            let status = pieces.next().ok_or(Error::InvalidEntryFormat)?.parse()?;
            let name = pieces.next().ok_or(Error::InvalidEntryFormat)?;

            // Update the entry if it exists or add a new entry
            match self.get_mut_by_id(id) {
                Some(entry) => {
                    entry.update(downloaded, status);
                }
                None => {
                    if percentage == "100%" {
                        self.entries.push(Entry::completed(
                            id,
                            downloaded,
                            status,
                            name.to_owned(),
                        ));
                    } else {
                        self.entries.push(Entry::from_id(id)?);
                    }
                }
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::{fs, path::Path};

    use crate::{bytes::Bytes, entry::Status};

    type BoxResult<T> = Result<T, Box<dyn std::error::Error>>;

    #[test]
    fn parse_entry_list() -> BoxResult<()> {
        let sample_file = Path::new("tests").join("corpus").join("entry_list.txt");
        let entry_list = fs::read_to_string(sample_file)?;

        let mut transmission = Transmission::empty();
        transmission.update_entries(&entry_list)?;

        let name = "archlinux-2021.04.01-x86_64.iso";
        let entry = Entry::completed(
            1,
            Bytes::from(786.8 * 1_024.0 * 1_024.0),
            Status::Seeding,
            name.to_owned(),
        );
        assert_eq!(transmission.entries, [entry.clone()]);
        assert_eq!(transmission.get_by_name(name), Some(&entry));

        Ok(())
    }
}
