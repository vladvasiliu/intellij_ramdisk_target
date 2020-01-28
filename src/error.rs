use std::error::Error as StdError;
use std::io;
use std::fmt;
use std::path::{Path, PathBuf};
use std::result::Result as StdResult;

pub type Result<T> = StdResult<T, Error>;


#[derive(Debug)]
pub enum ErrorKind {
    IOError(io::Error),
}

#[derive(Debug)]
pub struct Error {
    pub kind: ErrorKind,
    pub path: PathBuf,
}


impl Error {
    pub fn from_io_error(path: &Path, err: io::Error) -> Self {
        Self {
            path: path.into(),
            kind: ErrorKind::IOError(err),
        }
    }
}


impl StdError for Error {}


impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self.kind {
            ErrorKind::IOError(err) => write!(f, "I/O error reading `{}`: {}", self.path.display(), err),
        }
    }
}
