// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::io;
use std::fmt;
use std::error;
use std::str;

use nix;
use uuid;

#[derive(Debug, Clone)]
pub enum ErrorEnum {
    Error,

    AlreadyExists,
    Busy,
    Invalid,
    NotFound,
}

#[derive(Debug)]
pub enum EngineError {
    Engine(ErrorEnum, String),
    Io(io::Error),
    Nix(nix::Error),
    Uuid(uuid::ParseError),
    Utf8(str::Utf8Error),
}

impl fmt::Display for EngineError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            EngineError::Engine(_, ref msg) => write!(f, "Stratis error: {}", msg),
            EngineError::Io(ref err) => write!(f, "IO error: {}", err),
            EngineError::Nix(ref err) => write!(f, "Nix error: {}", err.errno().desc()),
            EngineError::Uuid(ref err) => write!(f, "Uuid error: {}", err),
            EngineError::Utf8(ref err) => write!(f, "Utf8 error: {}", err),
        }
    }
}

impl error::Error for EngineError {
    fn description(&self) -> &str {
        match *self {
            EngineError::Engine(_, ref msg) => msg,
            EngineError::Io(ref err) => err.description(),
            EngineError::Nix(ref err) => err.errno().desc(),
            EngineError::Uuid(_) => "Uuid::ParseError",
            EngineError::Utf8(ref err) => err.description(),
        }
    }
}

pub type EngineResult<T> = Result<T, EngineError>;

impl From<io::Error> for EngineError {
    fn from(err: io::Error) -> EngineError {
        EngineError::Io(err)
    }
}

impl From<nix::Error> for EngineError {
    fn from(err: nix::Error) -> EngineError {
        EngineError::Nix(err)
    }
}

impl From<uuid::ParseError> for EngineError {
    fn from(err: uuid::ParseError) -> EngineError {
        EngineError::Uuid(err)
    }
}

impl From<str::Utf8Error> for EngineError {
    fn from(err: str::Utf8Error) -> EngineError {
        EngineError::Utf8(err)
    }
}
