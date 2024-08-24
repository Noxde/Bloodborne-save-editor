use serde::{Deserialize, Serialize};

use std::path::PathBuf;

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
    pub fn build(save_path: &str, resources_path: PathBuf) -> Result<SaveData, Error> {
        let file = FileData::build(save_path, resources_path)?;
        let stats = stats::new(&file).unwrap();
        let inventory = inventory::build(&file);

        Ok(SaveData {
            file,
            stats,
            inventory,
        })
    }
}
