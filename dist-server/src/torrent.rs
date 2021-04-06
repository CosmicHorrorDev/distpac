use serde::Deserialize;
use tempfile::tempdir;

use std::{
    borrow::Cow,
    path::{Path, PathBuf},
    process::{Command, Stdio},
};

// TODO: configure output path for torrent file

#[derive(Deserialize)]
struct TorrentInfo {
    info_hash: String,
}

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

    pub fn create(path: &Path) -> Self {
        let output_dir = tempdir().unwrap();

        Self::create_at(path, output_dir.path())
    }

    pub fn create_at(path: &Path, dst_dir: &Path) -> Self {
        assert!(path.exists());

        let torrent_name = format!("{}.torrent", path.file_name().unwrap().to_str().unwrap());
        let torrent_path = dst_dir.join(&torrent_name);

        let escaped_path = shell_escape::escape(Cow::from(path.to_str().unwrap()));
        let escaped_torrent_path = shell_escape::escape(Cow::from(torrent_path.to_str().unwrap()));

        Command::new("imdl")
            .arg("torrent")
            .arg("create")
            .arg("--output")
            .arg(&*escaped_torrent_path)
            .arg(&*escaped_path)
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()
            .unwrap()
            .wait()
            .unwrap();

        let magnet = String::from_utf8(
            Command::new("imdl")
                .arg("torrent")
                .arg("link")
                .arg(&*escaped_torrent_path)
                .output()
                .unwrap()
                .stdout,
        )
        .expect("imdl returned invalid utf8")
        .trim()
        .to_string();

        let info_hash = serde_json::from_str::<TorrentInfo>(
            &String::from_utf8(
                Command::new("imdl")
                    .arg("torrent")
                    .arg("show")
                    .arg("--json")
                    .arg(&*escaped_torrent_path)
                    .output()
                    .unwrap()
                    .stdout,
            )
            .expect("imdl returned invalid utf8"),
        )
        .unwrap()
        .info_hash;

        Self::new(PathBuf::new(), magnet, info_hash)
    }
}
