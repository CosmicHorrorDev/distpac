use thiserror::Error;

use std::{io, string::FromUtf8Error};

#[derive(Error, Debug)]
pub enum Error {
    #[error("A byte string was in an unrecognized format")]
    InvalidByteFormat,
    #[error("An entry string was in an unrecognized format")]
    InvalidEntryFormat,
    #[error("A transmission command failed to execute Error: {0}")]
    CommandFailed(#[from] io::Error),
    #[error("A command return invalid UTF-8 Error: {0}")]
    InvalidUTF8(#[from] FromUtf8Error),
}
