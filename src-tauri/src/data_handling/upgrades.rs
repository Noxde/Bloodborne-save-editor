use serde::{Deserialize, Serialize};
use serde_json::{self,  Value};
use super::{enums::{UpgradeType, Error},
            file::FileData};
use std::{fs::File,
          io::BufReader,
          collections::HashMap,
          path::PathBuf};
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct UpgradeInfo {
    pub name: String,
    pub effect: String,
    pub rating: u8,
    pub level: u8,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Upgrade {
    pub id: u32,
    pub source: u32,
    pub upgrade_type: UpgradeType,
    pub shape: String,
    pub effects: [u32; 6],
    pub info: UpgradeInfo,
}

pub fn parse_upgrades(file_data: &FileData) -> HashMap<UpgradeType, Vec<Upgrade>> {
    let mut upgrades = HashMap::new();
    let (start, end) = file_data.offsets.upgrades;

    for i in (start .. end).step_by(40) {
        let id = u32::from_le_bytes([file_data.bytes[i + 0], file_data.bytes[i + 1], file_data.bytes[i + 2], file_data.bytes[i + 3]]);

        let source = u32::from_le_bytes([file_data.bytes[i + 4], file_data.bytes[i + 5], file_data.bytes[i + 6], file_data.bytes[i + 7]]);


        let upgrade_type = UpgradeType::try_from(file_data.bytes[i+8]).unwrap();

        let mut effects = [0; 6];
        effects[0] = u32::from_le_bytes([file_data.bytes[i + 16], file_data.bytes[i + 17], file_data.bytes[i + 18], file_data.bytes[i + 19]]);

        effects[1] = u32::from_le_bytes([file_data.bytes[i + 20], file_data.bytes[i + 21], file_data.bytes[i + 22], file_data.bytes[i + 23]]);

        effects[2] = u32::from_le_bytes([file_data.bytes[i + 24], file_data.bytes[i + 25], file_data.bytes[i + 26], file_data.bytes[i + 27]]);

        effects[3] = u32::from_le_bytes([file_data.bytes[i + 28], file_data.bytes[i + 29], file_data.bytes[i + 30], file_data.bytes[i + 31]]);

        effects[4] = u32::from_le_bytes([file_data.bytes[i + 32], file_data.bytes[i + 33], file_data.bytes[i + 34], file_data.bytes[i + 35]]);

        effects[5] = u32::from_le_bytes([file_data.bytes[i + 36], file_data.bytes[i + 37], file_data.bytes[i + 38], file_data.bytes[i + 39]]);


        let info = match get_info_upgrade(effects[0], upgrade_type, &file_data.resources_path) {
            Ok(inf) => inf,
            Err(_) => continue,
        };

        let shape = match get_shape(file_data.bytes[i+12], upgrade_type) {
            Ok(sha) => sha,
            Err(_) => continue,
        };

        let upgrade = Upgrade {
            id,
            source,
            upgrade_type,
            shape,
            effects,
            info,
        };
        let category = upgrades.entry(upgrade_type).or_insert(Vec::new());
        category.push(upgrade);
    };
    upgrades
}

pub fn get_info_upgrade(first_effect: u32, upgrade_type: UpgradeType, resources_path: &PathBuf) -> Result<UpgradeInfo, Error> {
    let file_path = resources_path.join("upgrades.json");
    let json_file =  File::open(file_path).map_err(Error::IoError)?;
    let reader = BufReader::new(json_file);
    let upgrades: Value = serde_json::from_reader(reader).unwrap();
    let upgrades = upgrades.as_object().unwrap();
    let upgrades = match upgrade_type {
        UpgradeType::Gem => upgrades["gemEffects"].as_object().unwrap(),
        UpgradeType::Rune => upgrades["runeEffects"].as_object().unwrap(),
    };

    match upgrades.keys().find(|x| x.parse::<u32>().unwrap() == first_effect) {
        Some(found) => {
            let info: UpgradeInfo = serde_json::from_value(upgrades[found].clone()).unwrap();
            return Ok(info)
        },
        None => ()
    }
    Err(Error::CustomError("ERROR: Failed to find info for the upgrade."))
}

pub fn get_shape(shape: u8, upgrade_type: UpgradeType) -> Result<String, Error> {
    if upgrade_type == UpgradeType::Gem {
        let res = match shape {
            0x01 => "Radial",
            0x02 => "Triangle",
            0x04 => "Waning",
            0x08 => "Circle",
            0x3F => "Droplet",
            _ => return Err(Error::CustomError("Invalid shape number.")),
        };
        Ok(res.to_string())
    } else {
        let res = match shape {
            0x01 => "-",
            0x02 => "Oath",
            _ => return Err(Error::CustomError("Invalid shape number.")),
        };
        Ok(res.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_parse_upgrades() {
        //TESTSAVE 0
        let file_data = FileData::build("saves/testsave0", PathBuf::from("resources")).unwrap();
        let upgrades = parse_upgrades(&file_data);
        let gems = upgrades.get(&UpgradeType::Gem).unwrap();
        let runes = upgrades.get(&UpgradeType::Rune).unwrap();
        assert_eq!(gems.len(), 1);
        assert_eq!(runes.len(), 1);

        //Item N0
        assert_eq!(gems[0].id, u32::from_le_bytes([0x41, 0x00, 0x80, 0xC0]));
        assert_eq!(gems[0].source, u32::from_le_bytes([0x26, 0x60, 0x01, 0x80]));
        assert_eq!(gems[0].upgrade_type, UpgradeType::Gem);
        assert_eq!(gems[0].effects, [0x440c,
                                     0x440c,
                                     0x440c,
                                     0x440c,
                                     0x440c,
                                     0x440c]);
        assert_eq!(gems[0].shape, String::from("Droplet"));
        let info = gems[0].info.clone();
        assert_eq!(info.name, String::from("Abyssal Blood Gem"));
        assert_eq!(info.effect, String::from("Adds physical ATK (+45)"));
        assert_eq!(info.rating, 20);
        assert_eq!(info.level, 7);

        //Item N0
        assert_eq!(runes[0].id, u32::from_le_bytes([0x42, 0x00, 0x80, 0xC0]));
        assert_eq!(runes[0].source, u32::from_le_bytes([0xBF, 0x92, 0x01, 0x80]));
        assert_eq!(runes[0].upgrade_type, UpgradeType::Rune);
        assert_eq!(runes[0].effects, [0x115582,
                                      0xffffffff,
                                      0xffffffff,
                                      0xffffffff,
                                      0xffffffff,
                                      0xffffffff]);
        assert_eq!(runes[0].shape, String::from("-"));
        let info = runes[0].info.clone();
        assert_eq!(info.name, String::from("Added to test. Please remove"));
        assert_eq!(info.effect, String::from("Remove this please"));
        assert_eq!(info.rating, 1);
        assert_eq!(info.level, 1);

        //TESTSAVE 7
        let file_data = FileData::build("saves/testsave7", PathBuf::from("resources")).unwrap();
        let upgrades = parse_upgrades(&file_data);
        let gems = upgrades.get(&UpgradeType::Gem).unwrap();
        assert!(upgrades.get(&UpgradeType::Rune).is_none());
        assert_eq!(gems.len(), 1);

        //Item N0
        assert_eq!(gems[0].id, u32::from_le_bytes([0x67, 0x00, 0x80, 0xC0]));
        assert_eq!(gems[0].source, u32::from_le_bytes([0xF0, 0x49, 0x02, 0x80]));
        assert_eq!(gems[0].upgrade_type, UpgradeType::Gem);
        assert_eq!(gems[0].effects, [0x2FB3BC,
                                     0x2E7754,
                                     0xffffffff,
                                     0xffffffff,
                                     0xffffffff,
                                     0xffffffff]);
        assert_eq!(gems[0].shape, String::from("Droplet"));
        let info = gems[0].info.clone();
        assert_eq!(info.name, String::from("Tempering Blood Gemstone (1)"));
        assert_eq!(info.effect, String::from("Physical ATK UP (+2.7%)"));
        assert_eq!(info.rating, 4);
        assert_eq!(info.level, 1);
    }
}