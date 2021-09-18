use std::{
    fmt::{self, Display, Formatter},
    result,
};

use crate::prelude::*;

#[derive(Debug)]
pub struct Error {
    details: String
}

impl Error {
    pub fn new(msg: &str) -> Error {
        Error{details: msg.to_string()}
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f,"{}",self.details)
    }
}

impl std::error::Error for Error {
    fn description(&self) -> &str {
        &self.details
    }
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Error {
        Error::new(&err.to_string())
    }
}

impl From<FileError> for Error {
    fn from(err: FileError) -> Self {
        Error::new(&err.to_string())
    }
}

impl From<FontError> for Error {
    fn from(err: FontError) -> Self {
        Error::new(&err.to_string())
    }
}

impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Self {
        Error::new(&err.to_string())
    }
}

pub type Result<T> = result::Result<T, Error>;