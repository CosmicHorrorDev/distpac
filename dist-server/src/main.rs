use imdl_wrapper::Torrent;

use std::path::Path;

fn main() {
    let torrent = Torrent::create(Path::new("/home/lovecraft/Downloads"));
    println!("{:#?}", torrent);
}
