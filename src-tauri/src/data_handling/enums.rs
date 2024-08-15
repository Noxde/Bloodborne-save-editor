use std::{fmt, io};
use serde::{Deserialize, Serialize};
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

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Hash, Eq, Copy)]
pub enum ArticleType {
    Consumable,
    Material,
    Key,
    Chalice,
    RightHand,
    LeftHand,
    Armor,
    Gem,
    Rune,
}

impl ArticleType {
    pub fn from_string(string: &str) -> ArticleType {
        match string {
            "consumables" => ArticleType::Consumable,
            "materials" => ArticleType::Material,
            "key" => ArticleType::Key,
            "chalices" => ArticleType::Chalice,
            "rightHand" => ArticleType::RightHand,
            "leftHand" => ArticleType::LeftHand,
            "armor" => ArticleType::Armor,
            "gem" => ArticleType::Gem,
            "rune" => ArticleType::Rune,
            _ => panic!("ERROR: Invalid category."),
        }
    }
}
