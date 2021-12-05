use std::{error, fmt, result, string::FromUtf8Error};

use macroquad::prelude::{FileError, FontError, ShaderError};

use crate::prelude::*;

enum Repr {
    Simple(ErrorKind),
    SimpleMessage(ErrorKind, &'static &'static str),
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
    pub(crate) fn as_str(&self) -> &'static str {
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

impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&self.repr, f)
    }
}

impl Error {
    pub fn new<E>(kind: ErrorKind, error: E) -> Error
    where
        E: Into<Box<dyn std::error::Error + Send + Sync>>,
    {
        Error {
            repr: Repr::Custom(Box::new(Custom {
                kind,
                error: error.into(),
            })),
        }
    }

    pub const fn new_const(kind: ErrorKind, msg: &'static &'static str) -> Self {
        Self {
            repr: Repr::SimpleMessage(kind, msg),
        }
    }

    pub fn kind(&self) -> ErrorKind {
        match self.repr {
            Repr::Custom(ref c) => c.kind,
            Repr::Simple(kind) => kind,
            Repr::SimpleMessage(kind, _) => kind,
        }
    }
}

impl From<ErrorKind> for Error {
    fn from(kind: ErrorKind) -> Self {
        Error {
            repr: Repr::Simple(kind),
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.repr {
            Repr::Custom(ref c) => c.error.fmt(fmt),
            Repr::Simple(kind) => write!(fmt, "{}", kind.as_str()),
            Repr::SimpleMessage(_, &msg) => msg.fmt(fmt),
        }
    }
}

impl fmt::Debug for Repr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Repr::Simple(kind) => f.debug_tuple("Kind").field(&kind).finish(),
            Repr::SimpleMessage(kind, &message) => f
                .debug_struct("Error")
                .field("kind", &kind)
                .field("message", &message)
                .finish(),
            Repr::Custom(ref c) => c.error.fmt(f),
        }
    }
}

impl error::Error for Error {
    #[allow(deprecated, deprecated_in_future)]
    fn description(&self) -> &str {
        match self.repr {
            Repr::Simple(kind) => kind.as_str(),
            Repr::SimpleMessage(_, &msg) => msg,
            Repr::Custom(ref c) => c.error.description(),
        }
    }

    #[allow(deprecated)]
    fn cause(&self) -> Option<&dyn error::Error> {
        match self.repr {
            Repr::Simple(..) => None,
            Repr::SimpleMessage(..) => None,
            Repr::Custom(ref c) => c.error.cause(),
        }
    }

    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self.repr {
            Repr::Simple(..) => None,
            Repr::SimpleMessage(..) => None,
            Repr::Custom(ref c) => c.error.source(),
        }
    }
}

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

impl From<crate::json::Error> for Error {
    fn from(err: crate::json::Error) -> Self {
        Error::new(ErrorKind::Parse, err)
    }
}

pub type Result<T> = result::Result<T, Error>;
