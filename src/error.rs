use std::{
    fmt,
    result,
    string::FromUtf8Error,
};

use macroquad::prelude::{
    FileError,
    ShaderError,
    FontError,
};

use crate::prelude::*;

#[derive(Debug)]
enum Repr {
    Message(ErrorKind, &'static &'static str),
    Custom(Box<Custom>),
}

#[derive(Debug)]
struct Custom {
    kind: ErrorKind,
    error: Box<dyn std::error::Error + Send + Sync>,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum ErrorKind {
    File,
    Parse,
    Material,
}

impl ErrorKind {
    pub(crate) fn to_str(&self) -> &'static str {
        match *self {
            ErrorKind::File => "file error",
            ErrorKind::Parse => "parse error",
            ErrorKind::Material => "material error",
        }
    }
}

pub struct Error {
    repr: Repr,
}

impl Error {
    pub fn new<E>(kind: ErrorKind, error: E) -> Error
        where E: Into<Box<dyn std::error::Error + Send + Sync>> {
        Error {
            repr: Repr::Custom(Box::new(Custom {
                kind,
                error: error.into(),
            }))
        }
    }

    pub fn from_str(kind: ErrorKind, message: &'static &'static str) -> Error {
        Error {
            repr: Repr::Message(kind, message),
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.repr {
            Repr::Message(_, &msg) => msg.fmt(fmt),
            Repr::Custom(ref c) => c.error.fmt(fmt),
        }
    }
}

impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&self.repr, f)
    }
}

impl std::error::Error for Error {}

impl From<io::Error> for Error {
    fn from(error: io::Error) -> Error {
        Error::new(ErrorKind::File, error)
    }
}

impl From<FromUtf8Error> for Error {
    fn from(error: FromUtf8Error) -> Self {
        Error::new(ErrorKind::Parse, error)
    }
}

impl From<FileError> for Error {
    fn from(error: FileError) -> Self {
        Error::new(ErrorKind::File, error)
    }
}

impl From<FontError> for Error {
    fn from(error: FontError) -> Self {
        Error::new(ErrorKind::Parse, error)
    }
}

impl From<ShaderError> for Error {
    fn from(error: ShaderError) -> Self {
        Error::new(ErrorKind::Material, error)
    }
}

impl From<serde_json::Error> for Error {
    fn from(error: serde_json::Error) -> Self {
        Error::new(ErrorKind::Parse, error)
    }
}

impl From<regex::Error> for Error {
    fn from(error: regex::Error) -> Self {
        Error::new(ErrorKind::Parse, error)
    }
}

pub type Result<T> = result::Result<T, Error>;