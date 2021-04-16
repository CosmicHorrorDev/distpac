use clap::Clap;
use log::debug;

use std::{net::SocketAddr, path::PathBuf};

/// A dead simple http server for serving files at whatever address is specified by `--socket`
#[derive(Clap, Debug)]
struct Opts {
    /// Silence all output
    #[clap(short, long)]
    quiet: bool,
    /// Increase verbosity
    #[clap(short, long, parse(from_occurrences))]
    verbose: usize,
    /// Specify the socket for the webserver to attempt to bind to
    #[clap(short, long, default_value = "127.0.0.1:9090")]
    socket: SocketAddr,
    /// The paths to the files that will be served at the root of the web server
    files: Vec<PathBuf>,
}

#[async_std::main]
async fn main() -> tide::Result<()> {
    let Opts {
        socket,
        files,
        verbose,
        quiet,
    } = Opts::parse();
    stderrlog::new()
        .quiet(quiet)
        .verbosity(verbose)
        .init()
        .unwrap();
    debug!("socket: {:?}", socket);
    debug!("files: {:#?}", files);

    let mut app = tide::new();
    for file in files {
        let file_name = file.file_name().unwrap().to_string_lossy();
        app.at(&format!("/{}", file_name)).serve_file(file)?;
    }
    app.listen(socket).await?;

    Ok(())
}
