use serde::{Deserialize, Serialize};

use super::{
    enums::Error,
    file::FileData,
    inventory::{self, Inventory},
    stats::{self, Stat},
};

#[derive(Serialize, Deserialize, Clone)]
pub struct SaveData {
    pub file: FileData,
    pub stats: Vec<Stat>,
    pub inventory: Inventory,
}

impl SaveData {
    pub fn build(path: &str, username: &str) -> Result<SaveData, Error> {
        let file = FileData::build(path, username)?;

        let stats = stats::new(&file).unwrap();
        let inventory = inventory::build(&file);

        Ok(SaveData {
            file,
            stats,
            inventory,
        })
    }
}
