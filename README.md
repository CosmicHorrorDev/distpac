## `distpac`

A proof of concept package manager backed by p2p transfers.

**Disclaimer: This is just a proof of concept and is not intended for actual usage**

## Design

`distpac` is meant to be a simplistic package manager composed of both a server and a client portion. The content distribution is set up to distribute packages over the BitTorrent protocol by using `transmission-remote` and `transmission-daemon`. From there the server simply runs a tracker, transmission to distribute all packages, and an HTTP server for distributing the package database (an SQLite file). The client uses the package database to get magnet links for different packages and downloads them with transmission.

## Installation

*Note: all testing for this was done on Arch Linux. It is possible that there are platform specific differences that can cause issues*

### Server

`dist-server` expects `opentracker`, `transmission-remote`, and `transmission-daemon` to all be installed on the server along with a recent Rust toolchain. The server specific software is `named-file-server` and `dist-server` so you can build and install from the project directory.

```text
$ cargo install --bin named-file-server
$ cargo install --bin dist-server
```

The server also needs a config file at `$XDG_DATA_HOME/distpac/server.yaml`. All this currently has is the announce url for the tracker being used like so:

```yaml
announce_url: http://tracker.address:6969/announce
```

### Client

`dist-client` expects just `transmission-remote` and `transmission-daemon` to be installed along with a recent Rust toolchain. The client can be built and installed from the project directory

```text
$ cargo install --bin dist-client
```

The client needs a config file at `$XDG_DATA_HOME/distpac/client.yaml`. This is for the url of the package database server like so:

```yaml
server_url: http://package.server
```

## Packages

A package is just a directory that follows a specific structure like so

```text
.
├── assets
│  └── …
├── manifest.yaml
└── scripts
   ├── install.sh
   └── uninstall.sh
```

The assets directory is intended to have the actual content for the package. The manifest file has some metadata on the package like the name and version number (only SemVer style versions are supported). An example manifest is as follows

```yaml
name: The-Package-Name
version: 1.2.3
```

There is also the `scripts` directory that has an install and uninstall script that is intended to be run for installing and uninstalling the package respectively.

## Server Overview

The server functions consist of starting and stopping different components as well as adding new packages. To start the components (seeder, tracker server, and database server) you use the `start` command like so

```text
$ dist-server start
```

You can also add packages by passing paths to different packages like so

```text
$ dist-server add /path/to/package1 /path/to/package2
```

### Client Overview

The client just consists of syncing the package database and installing and uninstalling packages. Syncing the database is just done with the `sync` command

```text
$ dist-client sync
```

While packages can be installed and uninstalled by their name

```text
$ dist-client install example-package
$ dist-client uninstall example-package
```
