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

impl Into<TypeFamily> for ArticleType {
    fn into(self) -> TypeFamily {
        match self {
            Self::Consumable => TypeFamily::Item,
            Self::Material => TypeFamily::Item,
            Self::Key => TypeFamily::Item,
            Self::Chalice => TypeFamily::Item,
            Self::RightHand => TypeFamily::Weapon,
            Self::LeftHand => TypeFamily::Weapon,
            Self::Armor => TypeFamily::Armor,
            Self::Gem => TypeFamily::Upgrade,
            Self::Rune => TypeFamily::Upgrade,
        }
    }
}
impl From<&str> for ArticleType {
    fn from(string: &str) -> ArticleType {
        match string {
            "consumable" => ArticleType::Consumable,
            "material" => ArticleType::Material,
            "key" => ArticleType::Key,
            "chalice" => ArticleType::Chalice,
            "rightHand" => ArticleType::RightHand,
            "leftHand" => ArticleType::LeftHand,
            "armor" => ArticleType::Armor,
            "gem" => ArticleType::Gem,
            "rune" => ArticleType::Rune,
            _ => panic!("ERROR: Invalid category."),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Copy)]
pub enum TypeFamily {
    Armor,
    Item,
    Upgrade,
    Weapon,
}
