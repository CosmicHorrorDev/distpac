use dist_utils::misc::stop_process_by_name;
use sysinfo::{System, SystemExt};

use std::{io, path::PathBuf, process::Command};

const DAEMON_NAME: &str = "transmission-daemon";
const REMOTE_NAME: &str = "transmission-remote";

pub struct Entry;

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
pub struct Transmission;

impl Transmission {
    pub fn start(opts: &TransmissionOpts) -> io::Result<Self> {
        if !Self::is_running() {
            let mut command = Command::new(DAEMON_NAME);

            // Add the download dir if specified
            if let Some(download_dir) = &opts.download_dir {
                command.arg("--download_dir").arg(&download_dir);
            }

            command.spawn()?;
        }

        Ok(Self)
    }

    pub fn from_running() -> Option<Self> {
        if Self::is_running() {
            Some(Self)
        } else {
            None
        }
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

    pub fn status(&self) -> Vec<Entry> {
        todo!()
    }
}
