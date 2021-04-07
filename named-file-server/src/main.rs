use clap::Clap;

use std::{
    net::{SocketAddr, SocketAddrV4},
    path::PathBuf,
};

/// A dead simple http server for serving files at whatever address is specified by `--socket`
#[derive(Clap, Debug)]
struct Opts {
    /// Specify the socket for the webserver to attempt to bind to
    #[clap(short, long, default_value = "127.0.0.1:9090")]
    socket: SocketAddrV4,
    /// The paths to the files that will be served at the root of the web server
    files: Vec<PathBuf>,
}

#[async_std::main]
async fn main() -> tide::Result<()> {
    let Opts { socket, files } = Opts::parse();

    let mut app = tide::new();
    for file in files {
        let file_name = file.file_name().unwrap().to_string_lossy();
        app.at(&format!("/{}", file_name)).serve_file(file)?;
    }
    app.listen(SocketAddr::from(socket)).await?;

    Ok(())
}
