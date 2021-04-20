use serde::Deserialize;

use std::{
    borrow::Cow,
    convert::TryFrom,
    path::Path,
    process::{Command, Stdio},
};

use crate::error::TorrentError;

#[derive(Deserialize)]
pub(crate) struct TorrentInfo {
    pub(crate) name: String,
    pub(crate) info_hash: String,
    pub(crate) content_size: u64,
}

impl TryFrom<&Path> for TorrentInfo {
    type Error = TorrentError;

    fn try_from(torrent_path: &Path) -> Result<Self, Self::Error> {
        if !torrent_path.is_file() {
            return Err(TorrentError::MissingTorrentFile(torrent_path.to_owned()));
        }

        let escaped_torrent_path = shell_escape::escape(Cow::from(torrent_path.to_str().unwrap()));

        let info_output = Command::new("imdl")
            .arg("torrent")
            .arg("show")
            .arg("--json")
            .arg(&*escaped_torrent_path)
            .output()
            .unwrap();
        if !info_output.status.success() {
            return Err(TorrentError::FailedExtractingInfoHash);
        }
        let info = serde_json::from_str::<TorrentInfo>(
            &String::from_utf8(info_output.stdout)
                .map_err(|_| TorrentError::FailedExtractingInfoHash)?,
        )
        .map_err(|_| TorrentError::FailedExtractingInfoHash)?;

        Ok(info)
    }
}

pub fn create_torrent(
    src_path: &Path,
    dst_dir: &Path,
    announce_url: &str,
) -> Result<(), TorrentError> {
    if !src_path.exists() {
        return Err(TorrentError::MissingInputPath(src_path.to_owned()));
    }

    if !dst_dir.is_dir() {
        return Err(TorrentError::DestinationMissingOrNotDir(dst_dir.to_owned()));
    }

    let escaped_path = shell_escape::escape(Cow::from(src_path.to_str().unwrap()));

    let torrent_name = format!(
        "{}.torrent",
        src_path.file_name().unwrap().to_str().unwrap()
    );
    let torrent_path = dst_dir.join(&torrent_name);
    let escaped_torrent_path = shell_escape::escape(Cow::from(torrent_path.to_str().unwrap()));

    // TODO: does this stuff really need to be escaped or will rust handle that for us?
    let creation_status = Command::new("imdl")
        .arg("torrent")
        .arg("create")
        .arg("--output")
        .arg(&*escaped_torrent_path)
        .arg("--announce")
        .arg(announce_url)
        .arg(&*escaped_path)
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .unwrap();
    if !creation_status.success() {
        return Err(TorrentError::FailedCreatingTorrent);
    }

    Ok(())
}

pub fn create_magnet_link(torrent_path: &Path) -> Result<String, TorrentError> {
    if !torrent_path.is_file() {
        return Err(TorrentError::MissingTorrentFile(torrent_path.to_owned()));
    }

    let escaped_torrent_path = shell_escape::escape(Cow::from(torrent_path.to_str().unwrap()));

    let magnet_output = Command::new("imdl")
        .arg("torrent")
        .arg("link")
        .arg(&*escaped_torrent_path)
        .output()
        .unwrap();
    if !magnet_output.status.success() {
        return Err(TorrentError::FailedGeneratingMagnent);
    }

    String::from_utf8(magnet_output.stdout).map_err(|_| TorrentError::FailedGeneratingMagnent)
}
