use std::{fs, io, path::PathBuf};

use crate::Mode;

pub fn create_dirs(mode: Mode) -> io::Result<()> {
    fs::create_dir_all(&database_dir())?;
    fs::create_dir_all(&torrent_data_dir())?;

    // Only the server has a separate location for torrent files. The client will just download
    // everything into the `torrent_data_dir`
    if let Mode::Server = mode {
        fs::create_dir_all(&torrent_file_dir())?;
    }

    Ok(())
}

pub fn server_config_file() -> PathBuf {
    base_dir().join("server.yaml")
}

pub fn client_config_file() -> PathBuf {
    base_dir().join("client.yaml")
}

pub fn installed_db_file() -> PathBuf {
    database_dir().join("installed.db")
}

pub fn package_db_file() -> PathBuf {
    database_dir().join("packages.db")
}

pub fn database_dir() -> PathBuf {
    base_dir().join("databases")
}

pub fn torrent_file_dir() -> PathBuf {
    torrent_dir().join("file")
}

pub fn torrent_data_dir() -> PathBuf {
    torrent_dir().join("data")
}

pub fn torrent_dir() -> PathBuf {
    base_dir().join("torrents")
}

pub fn base_dir() -> PathBuf {
    dirs_next::data_dir()
        .expect("Failed getting data dir")
        .join("distpac")
}
