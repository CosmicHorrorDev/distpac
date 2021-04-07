use clap::Clap;

use std::{
    net::{SocketAddr, SocketAddrV4},
    path::PathBuf,
};

/// A dead simple http server for serving a single file. The file will be served at the root of
/// whatever address is specified by `--socket`
#[derive(Clap, Debug)]
struct Opts {
    /// Specify the socket for the webserver to attempt to bind to
    #[clap(short, long, default_value = "127.0.0.1:9090")]
    socket: SocketAddrV4,
    /// The path to the file that will be served at the root of the web server
    file: PathBuf,
}

#[async_std::main]
async fn main() -> tide::Result<()> {
    let Opts { socket, file } = Opts::parse();

    let mut app = tide::new();
    app.at("/").serve_file(file)?;
    app.listen(SocketAddr::from(socket)).await?;

    Ok(())
}
