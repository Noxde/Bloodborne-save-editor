use std::{fmt, io};
use serde::{Deserialize, Serialize};
//use serde_json::Error as JsonError;

#[derive(Debug)]
pub enum Error {
    IoError(io::Error),
    CustomError(&'static str),
    //JsonError(JsonError),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::IoError(err) => write!(f, "I/0 error: {}",err),
            Error::CustomError(err) => write!(f, "Save error: {}",err),
            //Error::JsonError(err) => write!(f, "JSON SERDES error: {}",err),
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
            _ => panic!("ERROR: Invalid category."),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Copy)]
pub enum TypeFamily {
    Armor,
    Item,
    Weapon,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Copy)]
pub enum Imprint {
    Uncanny,
    Lost,
}

#[derive(PartialEq, Copy, Clone, Debug, Serialize, Deserialize)]
pub enum SlotShape {
    Closed,
    Radial,
    Triangle,
    Waning,
    Circle,
    Droplet,
}
impl TryFrom<&[u8; 4]> for SlotShape {
    type Error = Error;
    fn try_from(bytes: &[u8; 4]) -> Result<Self, Self::Error> {
        match bytes {
            [0x00, 0x00, 0x00, 0x80] => Ok(Self::Closed),
            [0x01, 0x00, 0x00, 0x00] => Ok(Self::Radial),
            [0x02, 0x00, 0x00, 0x00] => Ok(Self::Triangle),
            [0x04, 0x00, 0x00, 0x00] => Ok(Self::Waning),
            [0x08, 0x00, 0x00, 0x00] => Ok(Self::Circle),
            [0x3F, 0x00, 0x00, 0x00] => Ok(Self::Droplet),
            _ => Err(Error::CustomError("ERROR: Invalid shape.")),
        }
    }
}
impl Into<[u8; 4]> for SlotShape {
    fn into(self) -> [u8; 4] {
        match self {
            Self::Closed => [0x00, 0x00, 0x00, 0x80],
            Self::Radial => [0x01, 0x00, 0x00, 0x00],
            Self::Triangle => [0x02, 0x00, 0x00, 0x00],
            Self::Waning => [0x04, 0x00, 0x00, 0x00],
            Self::Circle => [0x08, 0x00, 0x00, 0x00],
            Self::Droplet => [0x3F, 0x00, 0x00, 0x00],
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Copy, Hash)]
pub enum UpgradeType{
    Gem,
    Rune,
}

impl TryFrom<u8> for UpgradeType {
    type Error = Error;
    fn try_from(number: u8) -> Result<Self, Self::Error> {
        match number {
            0x01 => Ok(UpgradeType::Gem),
            0x02 => Ok(UpgradeType::Rune),
            _ => Err(Error::CustomError("ERROR: Invalid type.")),
        }
    }
}

#[derive(Deserialize)]
pub enum Location {
    Inventory,
    Storage,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic(expected = "ERROR: Invalid category.")]
    fn test_article_type_from_string() {
        let _ = ArticleType::from("error");
    }

    #[test]
    fn test_upgrade_type_try_from_u8() {
        let result = UpgradeType::try_from(255);
        assert!(result.is_err());
        if let Err(error) = result {
            assert_eq!(error.to_string(), "Save error: ERROR: Invalid type.");
        }
    }
}
