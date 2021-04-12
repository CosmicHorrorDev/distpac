use std::{fs, io, path::PathBuf};

pub fn create_dirs() -> io::Result<()> {
    fs::create_dir_all(&database_dir())?;
    fs::create_dir_all(&torrent_file_dir())?;
    fs::create_dir_all(&torrent_data_dir())?;

    Ok(())
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
