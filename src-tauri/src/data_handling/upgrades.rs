use serde::{Deserialize, Serialize};
use super::{enums::UpgradeType,
            file::FileData};
use std::collections::HashMap;
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct UpgradeInfo {
    pub name: String,
    pub effect: String,
    pub shape: String,
    pub rating: u8,
    pub level: u8,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Upgrade {
    pub id: u32,
    pub source: u32,
    pub upgrade_type: UpgradeType,
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

        //TODO: get_info_gem() and get_info_rune()
        let info = UpgradeInfo {
            name: String::from("Name"),
            effect: String::from("effect"),
            shape: String::from("shape"),
            rating: 0,
            level: 0,
        };

        let upgrade = Upgrade {
            id,
            source,
            upgrade_type,
            effects,
            info,
        };
        let category = upgrades.entry(upgrade_type).or_insert(Vec::new());
        category.push(upgrade);
    };
    upgrades
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_parse_upgrades() {
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
        //assert_eq!(gems[0].info, ...);

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
        //assert_eq!(gems[0].info, ...);

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
        //assert_eq!(gems[0].info, ...);
    }
}
