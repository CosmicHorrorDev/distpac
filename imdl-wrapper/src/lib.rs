use std::path::{Path, PathBuf};

use crate::error::TorrentError;

mod error;
mod utils;

#[derive(Debug)]
pub struct Torrent {
    pub path: PathBuf,
    pub magnet: String,
    pub info_hash: String,
}

impl Torrent {
    fn new(path: PathBuf, magnet: String, info_hash: String) -> Self {
        Self {
            path,
            magnet,
            info_hash,
        }
    }

    pub fn create_at(src_path: &Path, dst_dir: &Path) -> Result<Self, TorrentError> {
        let torrent_name = format!(
            "{}.torrent",
            src_path.file_name().unwrap().to_str().unwrap()
        );
        let torrent_path = dst_dir.join(&torrent_name);

        utils::create_torrent(src_path, dst_dir)?;
        let magnet = utils::create_magnet_link(&torrent_path)?;
        let info_hash = utils::get_info_hash(&torrent_path)?;

        Ok(Self::new(PathBuf::new(), magnet, info_hash))
    }
}
