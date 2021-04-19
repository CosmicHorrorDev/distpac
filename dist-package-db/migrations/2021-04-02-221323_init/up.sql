-- Your SQL goes here
CREATE TABLE packages (
    torrent_name VARCHAR UNIQUE NOT NULL,
    name VARCHAR NOT NULL,
    version INTEGER NOT NULL,
    magnet VARCHAR NOT NULL,
    size_bytes INTEGER NOT NULL,
    PRIMARY KEY (name, version)
)
