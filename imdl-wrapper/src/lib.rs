use std::{
    convert::TryFrom,
    path::{Path, PathBuf},
};

use crate::{error::TorrentError, utils::TorrentInfo};

pub mod error;
mod utils;

#[derive(Debug)]
pub struct Torrent {
    pub name: String,
    pub path: PathBuf,
    pub magnet: String,
    pub info_hash: String,
    pub size: u64,
}

impl Torrent {
    fn new(name: String, path: PathBuf, magnet: String, info_hash: String, size: u64) -> Self {
        Self {
            name,
            path,
            magnet,
            info_hash,
            size,
        }
    }

    pub fn create(
        src_path: &Path,
        dst_dir: &Path,
        announce_url: &str,
    ) -> Result<Self, TorrentError> {
        let torrent_name = format!(
            "{}.torrent",
            src_path.file_name().unwrap().to_str().unwrap()
        );
        let torrent_path = dst_dir.join(&torrent_name);

        utils::create_torrent(src_path, dst_dir, announce_url)?;
        let magnet = utils::create_magnet_link(&torrent_path)?;
        let TorrentInfo {
            name,
            info_hash,
            content_size,
        } = TorrentInfo::try_from(torrent_path.as_path())?;

        Ok(Self::new(
            name,
            torrent_path,
            magnet,
            info_hash,
            content_size,
        ))
    }
}
