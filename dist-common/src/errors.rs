use thiserror::Error;

#[derive(Error, Debug)]
pub enum ParseVersionError {
    #[error("Expected three values, but got {0} instead")]
    InvalidValuesCount(usize),
    #[error("Expected valid u16, but got {0} instead")]
    InvalidValue(String),
}

impl ParseVersionError {
    pub fn invalid_count(count: usize) -> Self {
        Self::InvalidValuesCount(count)
    }

    pub fn invalid_value(value: String) -> Self {
        Self::InvalidValue(value)
    }
}

#[derive(Error, Debug)]
pub enum DatabaseError {
    #[error("Failed connecting to package database with {0}")]
    FailedConnnection(#[from] diesel::result::Error),
    #[error("Failed inserting value into package database with {0}")]
    FailedInsertion(#[from] diesel::result::Error),
}
