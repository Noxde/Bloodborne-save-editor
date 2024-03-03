use std::{fmt, io};
use fltk::prelude::FltkError;

#[derive(Debug)]
pub enum Error {
    IoError(io::Error),
    CustomError(&'static str),
    UiError(FltkError),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::IoError(err) => write!(f, "I/0 error: {}",err),
            Error::CustomError(err) => write!(f, "Save error: {}",err),
            Error::UiError(err) => write!(f, "UI error: {}",err),
        }
    }
}

impl std::error::Error for Error {}