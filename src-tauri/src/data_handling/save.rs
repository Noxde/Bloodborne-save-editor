use serde::{Deserialize, Serialize};

use std::{path::PathBuf,
          collections::HashMap};

use super::{
    enums::{Error, UpgradeType},
    file::FileData,
    inventory::{self, Inventory},
    stats::{self, Stat},
    upgrades::{Upgrade, parse_upgrades},
    username::Username,
};

#[derive(Serialize, Deserialize, Clone)]
pub struct SaveData {
    pub file: FileData,
    pub stats: Vec<Stat>,
    pub inventory: Inventory,
    pub storage: Inventory,
    pub upgrades: HashMap<UpgradeType, Vec<Upgrade>>,
    pub username: Username,
}

impl SaveData {
    pub fn build(save_path: &str, resources_path: PathBuf) -> Result<SaveData, Error> {
        let file = FileData::build(save_path, resources_path)?;
        let stats = stats::new(&file).unwrap();
        let inventory = inventory::build(&file, file.offsets.inventory, file.offsets.key_inventory);
        let storage = inventory::build(&file, file.offsets.storage, (0,0)); // Its not possible to store key items
        let upgrades = parse_upgrades(&file);
        let username = Username::build(&file);

        Ok(SaveData {
            file,
            stats,
            inventory,
            storage,
            upgrades,
            username,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build() {
        assert!(SaveData::build("saves/testsave0", PathBuf::from("resources")).is_ok());
    }

}
