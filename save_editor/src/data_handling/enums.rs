use std::{fmt, io};
use serde_json::Error as JsonError;

#[derive(Debug)]
pub enum Error {
    IoError(io::Error),
    CustomError(&'static str),
    JsonError(JsonError),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::IoError(err) => write!(f, "I/0 error: {}",err),
            Error::CustomError(err) => write!(f, "Save error: {}",err),
            Error::JsonError(err) => write!(f, "JSON SERDES error: {}",err),
        }
    }
}

impl std::error::Error for Error {}

#[derive(Debug)]
pub enum ArticleType {
    Armor,
    Item,
    Weapon,
}