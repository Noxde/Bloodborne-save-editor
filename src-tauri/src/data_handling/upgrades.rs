use serde::{Deserialize, Serialize};
use serde_json::{self,  Value};
use super::{enums::{UpgradeType, Error},
            file::FileData};
use std::{fs::File,
          io::BufReader,
          collections::HashMap};
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct UpgradeInfo {
    pub name: String,
    pub effect: String,
    pub rating: u8,
    pub level: u8,
    pub note: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Upgrade {
    pub number: u8,
    pub id: u32,
    pub source: u32,
    pub upgrade_type: UpgradeType,
    pub shape: String,
    pub effects: Vec<(u32, String)>,
    pub info: UpgradeInfo,
    pub index: usize, //Index of the upgrade inside the vector
}

impl Upgrade {
    pub fn change_shape(&mut self, file_data: &mut FileData, new_shape: String) -> Result<(), Error> {
        let new_shape_number: u8 = match self.upgrade_type {
            UpgradeType::Gem => {
                match new_shape.as_str() {
                    "Radial" => 0x01,
                    "Triangle" => 0x02,
                    "Waning" => 0x04,
                    "Circle" => 0x08,
                    "Droplet" => 0x3F,
                    _ => return Err(Error::CustomError("Invalid shape.")),
                }
            },
            UpgradeType::Rune => {
                match new_shape.as_str() {
                    "-" => 0x01,
                    "Oath" => 0x02,
                    _ => return Err(Error::CustomError("Invalid shape.")),
                }
            },
        };

        let upgrade_offset = match file_data.find_upgrade_offset(self.id) {
            Some(offset) => offset,
            None => return Err(Error::CustomError("Failed to find the upgrade in the file data.")),
        };

        //Update the shape
        self.shape = new_shape;
        file_data.bytes[upgrade_offset+12] = new_shape_number;
        Ok(())
    }

    //value_index must be 0..=5
    pub fn change_effect(&mut self, file_data: &mut FileData, new_value: u32, value_index: usize) -> Result<(), Error> {
        let file_path = file_data.resources_path.join("upgrades.json");
        let json_file =  File::open(file_path).map_err(Error::IoError).unwrap();
        let reader = BufReader::new(json_file);
        let upgrades_json: Value = serde_json::from_reader(reader).unwrap();

        let upgrade_offset = match file_data.find_upgrade_offset(self.id) {
            Some(offset) => offset,
            None => return Err(Error::CustomError("Failed to find the upgrade in the file data.")),
        };

        let fallback: &Value;
        let json_effects: &Value = match self.upgrade_type {
            UpgradeType::Gem => {
                fallback = &upgrades_json["runeEffects"];
                &upgrades_json["gemEffects"]
            }
            UpgradeType::Rune => {
                fallback = &upgrades_json["gemEffects"];
                &upgrades_json["runeEffects"]
            }
        };

        let mut json_effect = &json_effects[new_value.to_string()];
        //If the new effect wasn't found, search in the other upgrade type's effects
        if json_effect.is_null() {
            json_effect = &fallback[new_value.to_string()];
        }

        let effect_info: UpgradeInfo = match serde_json::from_value(json_effect.clone()) {
            Ok(inf) => inf,
            Err(_) => return Err(Error::CustomError("Failed to find information of the new effect.")),
        };
        match self.effects.get_mut(value_index) {
            Some(e) => {
                e.0 = new_value;
                e.1 = effect_info.effect.clone();
            },
            None => return Err(Error::CustomError("Invalid index.")),
        };

        if value_index == 0 {
            self.info = effect_info;
        }

        let effect_offset = upgrade_offset + 16 + (value_index * 4);
        let bytes = new_value.to_le_bytes();

        for i in effect_offset .. effect_offset + 4 {
            file_data.bytes[i] = bytes[i-effect_offset];
        }
        Ok(())
    }

    pub fn transform(&mut self, file_data: &mut FileData) -> Result<(), Error> {
        let new_upgrade_bytes: [u8; 32] = match self.upgrade_type {
            UpgradeType::Gem => {
                self.upgrade_type = UpgradeType::Rune;
                self.shape = String::from("-");
                self.effects = vec![(1100000, String::from("More echoes from slain enemies 1")),
                                (0xFFFFFFFF, String::from("No Effect")),
                                (0xFFFFFFFF, String::from("No Effect")),
                                (0xFFFFFFFF, String::from("No Effect")),
                                (0xFFFFFFFF, String::from("No Effect")),
                                (0xFFFFFFFF, String::from("No Effect"))];
                self.info.name = String::from("Moon");
                self.info.effect = String::from("More echoes from slain enemies 1");
                self.info.rating = 0;
                self.info.level = 0;
                self.info.note = String::from("\"Moon\" rune. Acquire more Blood Echoes");
                [0x02, 0x00, 0x00, 0x00, //Type
                 0x01, 0x00, 0x00, 0x00, //Shape
                 0xE0, 0xC8, 0x10, 0x00, //Effect 1
                 0xFF, 0xFF, 0xFF, 0xFF, //Effect 2
                 0xFF, 0xFF, 0xFF, 0xFF, //Effect 3
                 0xFF, 0xFF, 0xFF, 0xFF, //Effect 4
                 0xFF, 0xFF, 0xFF, 0xFF, //Effect 5
                 0xFF, 0xFF, 0xFF, 0xFF] //Effect 6
            },
            UpgradeType::Rune => {
                self.upgrade_type = UpgradeType::Gem;
                self.shape = String::from("Radial");
                self.effects = vec![(13101, String::from("Add blood ATK +0.5")),
                                (0xFFFFFFFF, String::from("No Effect")),
                                (0xFFFFFFFF, String::from("No Effect")),
                                (0xFFFFFFFF, String::from("No Effect")),
                                (0xFFFFFFFF, String::from("No Effect")),
                                (0xFFFFFFFF, String::from("No Effect"))];
                self.info.name = String::from("Bloodtinge Blood Gemstone (1)");
                self.info.effect = String::from("Add blood ATK +0.5");
                self.info.rating = 1;
                self.info.level = 1;
                self.info.note = String::from("");
                [0x01, 0x00, 0x00, 0x00, //Type
                 0x01, 0x00, 0x00, 0x00, //Shape
                 0x2D, 0x33, 0x00, 0x00, //Effect 1
                 0xFF, 0xFF, 0xFF, 0xFF, //Effect 2
                 0xFF, 0xFF, 0xFF, 0xFF, //Effect 3
                 0xFF, 0xFF, 0xFF, 0xFF, //Effect 4
                 0xFF, 0xFF, 0xFF, 0xFF, //Effect 5
                 0xFF, 0xFF, 0xFF, 0xFF] //Effect 6
            },
        };
        let upgrade_offset = match file_data.find_upgrade_offset(self.id) {
            Some(offset) => offset,
            None => return Err(Error::CustomError("Failed to find the upgrade in the file data.")),
        };

        //Update everything except id and source
        for i in upgrade_offset + 8  .. upgrade_offset + 40 {
            file_data.bytes[i] = new_upgrade_bytes[i-8-upgrade_offset];
        }
        Ok(())
    }
}

pub fn parse_upgrades(file_data: &FileData) -> HashMap<u32, (Upgrade, UpgradeType)> {
    let mut upgrades = HashMap::new();
    let file_path = file_data.resources_path.join("upgrades.json");
    let json_file =  File::open(file_path).map_err(Error::IoError).unwrap();
    let reader = BufReader::new(json_file);
    let upgrades_json: Value = serde_json::from_reader(reader).unwrap();

    let (start, end) = file_data.offsets.upgrades;

    for i in (start .. end).step_by(40) {
        let id = u32::from_le_bytes([file_data.bytes[i + 0], file_data.bytes[i + 1], file_data.bytes[i + 2], file_data.bytes[i + 3]]);

        let source = u32::from_le_bytes([file_data.bytes[i + 4], file_data.bytes[i + 5], file_data.bytes[i + 6], file_data.bytes[i + 7]]);


        let upgrade_type = UpgradeType::try_from(file_data.bytes[i+8]).unwrap();

        let mut effects: Vec<(u32, String)> = Vec::with_capacity(6);
        let mut effects_ids = [0; 6];
        effects_ids[0] = u32::from_le_bytes([file_data.bytes[i + 16], file_data.bytes[i + 17], file_data.bytes[i + 18], file_data.bytes[i + 19]]);

        effects_ids[1] = u32::from_le_bytes([file_data.bytes[i + 20], file_data.bytes[i + 21], file_data.bytes[i + 22], file_data.bytes[i + 23]]);

        effects_ids[2] = u32::from_le_bytes([file_data.bytes[i + 24], file_data.bytes[i + 25], file_data.bytes[i + 26], file_data.bytes[i + 27]]);

        effects_ids[3] = u32::from_le_bytes([file_data.bytes[i + 28], file_data.bytes[i + 29], file_data.bytes[i + 30], file_data.bytes[i + 31]]);

        effects_ids[4] = u32::from_le_bytes([file_data.bytes[i + 32], file_data.bytes[i + 33], file_data.bytes[i + 34], file_data.bytes[i + 35]]);

        effects_ids[5] = u32::from_le_bytes([file_data.bytes[i + 36], file_data.bytes[i + 37], file_data.bytes[i + 38], file_data.bytes[i + 39]]);


        let fallback: &Value;
        let json_effects: &Value = match upgrade_type {
            UpgradeType::Gem => {
                fallback = &upgrades_json["runeEffects"];
                &upgrades_json["gemEffects"]
            }
            UpgradeType::Rune => {
                fallback = &upgrades_json["gemEffects"];
                &upgrades_json["runeEffects"]
            }
        };

        let mut info = UpgradeInfo {
            name: String::from(""),
            effect: String::from("No Effect"),
            rating: 95,
            level: 0,
            note: String::from(""),
        };

        let mut is_cursed = false; // Initialize the is_cursed flag

        for e in 0 .. 6 {
            let mut json_effect = &json_effects[&effects_ids[e].to_string()];
            //If the effect wasn't found, search in the other upgrade type's effects
            if json_effect.is_null() {
                json_effect = &fallback[&effects_ids[e].to_string()];
            }

            let effect_info: UpgradeInfo = match serde_json::from_value(json_effect.clone()) {
                Ok(inf) => inf,
                Err(_) => continue,
            };
            if upgrade_type == UpgradeType::Gem && effect_info.effect.contains("-") {
                is_cursed = true; // Set the flag if "Cursed" is found
            }
            effects.push((effects_ids[e], effect_info.effect.clone()));
            if e == 0 {
                info = effect_info;
            }

        }

        if is_cursed {
            info.name = format!("Cursed {}", info.name); // Prefix "Cursed" to the name if any effect is cursed
        }

        let shape = match get_shape(file_data.bytes[i+12], upgrade_type) {
            Ok(sha) => sha,
            Err(_) => continue,
        };

        let upgrade = Upgrade {
            number: 0, //It will be set correctly in Inventory::build()

            id,
            source,
            upgrade_type,
            shape,
            effects,
            info,
            index: 0, //It will be set correctly in Inventory::build()
        };
        upgrades.insert(id, (upgrade, upgrade_type));
    };
    upgrades
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
    use std::{path::PathBuf,
              time::Instant,
              thread};

    #[test]
    fn test_parse_upgrades() {
        //TESTSAVE 0
        let file_data = FileData::build("saves/testsave0", PathBuf::from("resources")).unwrap();
        let upgrades = parse_upgrades(&file_data);

        //Item N0
        let gem = upgrades.get(&u32::from_le_bytes([0x41, 0x00, 0x80, 0xC0])).unwrap().0.clone();
        assert_eq!(gem.id, u32::from_le_bytes([0x41, 0x00, 0x80, 0xC0]));
        assert_eq!(gem.source, u32::from_le_bytes([0x26, 0x60, 0x01, 0x80]));
        assert_eq!(gem.upgrade_type, UpgradeType::Gem);
        assert_eq!(gem.effects, vec![(0x440c, String::from("Add physical ATK +45")),
                                         (0x440c, String::from("Add physical ATK +45")),
                                         (0x440c, String::from("Add physical ATK +45")),
                                         (0x440c, String::from("Add physical ATK +45")),
                                         (0x440c, String::from("Add physical ATK +45")),
                                         (0x440c, String::from("Add physical ATK +45"))]);
        assert_eq!(gem.shape, String::from("Droplet"));
        let info = gem.info.clone();
        assert_eq!(info.name, String::from("Tempering Abyssal Blood Gem"));
        assert_eq!(info.effect, String::from("Add physical ATK +45"));
        assert_eq!(info.rating, 20);
        assert_eq!(info.level, 7);
        assert_eq!(info.note, String::from(""));

        //Item N0
        let rune = upgrades.get(&u32::from_le_bytes([0x42, 0x00, 0x80, 0xC0])).unwrap().0.clone();
        assert_eq!(rune.id, u32::from_le_bytes([0x42, 0x00, 0x80, 0xC0]));
        assert_eq!(rune.source, u32::from_le_bytes([0xBF, 0x92, 0x01, 0x80]));
        assert_eq!(rune.upgrade_type, UpgradeType::Rune);
        assert_eq!(rune.effects, vec![(0x115582, String::from("Max QS bullets held UP +3")),
                                          (0xffffffff, String::from("No Effect")),
                                          (0xffffffff, String::from("No Effect")),
                                          (0xffffffff, String::from("No Effect")),
                                          (0xffffffff, String::from("No Effect")),
                                          (0xffffffff, String::from("No Effect"))]);
        assert_eq!(rune.shape, String::from("-"));
        let info = rune.info.clone();
        assert_eq!(info.name, String::from("Formless Oedon"));
        assert_eq!(info.effect, String::from("Max QS bullets held UP +3"));
        assert_eq!(info.rating, 2);
        assert_eq!(info.level, 0);
        assert_eq!(info.note, String::from("Higher Quicksilver Bullet max"));

        //TESTSAVE 7
        let file_data = FileData::build("saves/testsave7", PathBuf::from("resources")).unwrap();
        let upgrades = parse_upgrades(&file_data);

        //Item N0
        let gem = upgrades.get(&u32::from_le_bytes([0x67, 0x00, 0x80, 0xC0])).unwrap().0.clone();
        assert_eq!(gem.id, u32::from_le_bytes([0x67, 0x00, 0x80, 0xC0]));
        assert_eq!(gem.source, u32::from_le_bytes([0xF0, 0x49, 0x02, 0x80]));
        assert_eq!(gem.upgrade_type, UpgradeType::Gem);
        assert_eq!(gem.effects, vec![(0x2FB3BC, String::from("Physical ATK UP +2.7%")),
                                         (0x2E7754, String::from("Boosts rally potential +1.8%")),
                                         (0xffffffff, String::from("No Effect")),
                                         (0xffffffff, String::from("No Effect")),
                                         (0xffffffff, String::from("No Effect")),
                                         (0xffffffff, String::from("No Effect"))]);
        assert_eq!(gem.shape, String::from("Droplet"));
        let info = gem.info.clone();
        assert_eq!(info.name, String::from("Tempering Blood Gemstone (1)"));
        assert_eq!(info.effect, String::from("Physical ATK UP +2.7%"));
        assert_eq!(info.rating, 4);
        assert_eq!(info.level, 1);
        assert_eq!(info.note, String::from(""));
    }

    #[test]
    #[ignore] //cargo test -- --include-ignored
    fn test_parse_upgrades_runtime() {

        //TESTSAVE 0
        let handle0 = thread::spawn(|| {
            let file_data = FileData::build("saves/testsave0", PathBuf::from("resources")).unwrap();
            let now = Instant::now();
            parse_upgrades(&file_data);
            let elapsed = now.elapsed().as_millis();
            assert!(elapsed < 1000);
        });

        //TESTSAVE 1
        let handle1 = thread::spawn(|| {
            let file_data = FileData::build("saves/testsave1", PathBuf::from("resources")).unwrap();
            let now = Instant::now();
            parse_upgrades(&file_data);
            let elapsed = now.elapsed().as_millis();
            assert!(elapsed < 10000);
        });

        //TESTSAVE 2
        let handle2 = thread::spawn(|| {
            let file_data = FileData::build("saves/testsave2", PathBuf::from("resources")).unwrap();
            let now = Instant::now();
            parse_upgrades(&file_data);
            let elapsed = now.elapsed().as_millis();
            assert!(elapsed < 9000);
        });

        //TESTSAVE 3
        let handle3 = thread::spawn(|| {
            let file_data = FileData::build("saves/testsave3", PathBuf::from("resources")).unwrap();
            let now = Instant::now();
            parse_upgrades(&file_data);
            let elapsed = now.elapsed().as_millis();
            assert!(elapsed < 20000);
        });

        handle0.join().unwrap();
        handle1.join().unwrap();
        handle2.join().unwrap();
        handle3.join().unwrap();
    }


    #[test]
    fn upgrade_change_shape() {
        let mut file_data = FileData::build("saves/testsave3", PathBuf::from("resources")).unwrap();
        let upgrades1 = parse_upgrades(&file_data);

        //Droplet
        let gem1_1 = upgrades1.get(&3229615809).unwrap().0.clone();
        //Radial
        let gem1_2 = upgrades1.get(&3229615814).unwrap().0.clone();
        //Oath
        let rune1_1 = upgrades1.get(&3229615805).unwrap().0.clone();
        //-
        let rune1_2 = upgrades1.get(&3229615823).unwrap().0.clone();

        //Droplet
        let mut gem2_1 = upgrades1.get(&3229615809).unwrap().0.clone();
        //Radial
        let mut gem2_2 = upgrades1.get(&3229615814).unwrap().0.clone();
        //Oath
        let mut rune2_1 = upgrades1.get(&3229615805).unwrap().0.clone();
        //-
        let mut rune2_2 = upgrades1.get(&3229615823).unwrap().0.clone();

        //Run the function
        let result = gem2_1.change_shape(&mut file_data, String::from("Test error"));
        assert!(result.is_err());
        if let Err(e) = result {
            assert_eq!(e.to_string(), "Save error: Invalid shape.");
        }
        gem2_1.change_shape(&mut file_data, String::from("Waning")).unwrap();
        gem2_2.change_shape(&mut file_data, String::from("Triangle")).unwrap();

        let result = rune2_1.change_shape(&mut file_data, String::from("Test error"));
        assert!(result.is_err());
        if let Err(e) = result {
            assert_eq!(e.to_string(), "Save error: Invalid shape.");
        }
        rune2_1.change_shape(&mut file_data, String::from("-")).unwrap();
        rune2_2.change_shape(&mut file_data, String::from("Oath")).unwrap();

        //Compare
        let check = |upgrade_a: Upgrade, upgrade_b: Upgrade| -> bool {
            (upgrade_a.id == upgrade_b.id) &&
            (upgrade_a.source == upgrade_b.source) &&
            (upgrade_a.upgrade_type == upgrade_b.upgrade_type) &&
            (upgrade_a.effects == upgrade_b.effects) &&
            (upgrade_a.info == upgrade_b.info)
        };
        assert_eq!(gem2_1.shape, "Waning");
        assert!(check(gem1_1, gem2_1.clone()));

        assert_eq!(gem2_2.shape, "Triangle");
        assert!(check(gem1_2, gem2_2.clone()));

        assert_eq!(rune2_1.shape, "-");
        assert!(check(rune1_1, rune2_1.clone()));

        assert_eq!(rune2_2.shape, "Oath");
        assert!(check(rune1_2, rune2_2.clone()));

        let upgrades2 = parse_upgrades(&file_data);

        //Waning
        let gem3_1 = upgrades2.get(&3229615809).unwrap().0.clone();
        //Triangle
        let gem3_2 = upgrades2.get(&3229615814).unwrap().0.clone();
        //-
        let rune3_1 = upgrades2.get(&3229615805).unwrap().0.clone();
        //Oath
        let rune3_2 = upgrades2.get(&3229615823).unwrap().0.clone();

        assert_eq!(gem2_1, gem3_1);
        assert_eq!(gem2_2, gem3_2);
        assert_eq!(rune2_1, rune3_1);
        assert_eq!(rune2_2, rune3_2);

        gem2_1.id = 0;
        let result = gem2_1.change_shape(&mut file_data, String::from("Radial"));
        assert!(result.is_err());
        if let Err(e) = result {
            assert_eq!(e.to_string(), "Save error: Failed to find the upgrade in the file data.");
        }
    }

    #[test]
    fn upgrade_change_effect() {
        //TESTSAVE 0
        let mut file_data = FileData::build("saves/testsave0", PathBuf::from("resources")).unwrap();
        let upgrades = parse_upgrades(&file_data);
        let gem = upgrades.get(&3229614145).unwrap().0.clone();
        let mut gem2 = gem.clone();
        let rune = upgrades.get(&3229614146).unwrap().0.clone();
        let mut rune2 = rune.clone();

        let result = gem2.change_effect(&mut file_data, 0x00, 0);
        assert!(result.is_err());
        if let Err(e) = result {
            assert_eq!(e.to_string(), "Save error: Failed to find information of the new effect.");
        }

        let result = rune2.change_effect(&mut file_data, 0xFFFFFFFF, 9);
        assert!(result.is_err());
        if let Err(e) = result {
            assert_eq!(e.to_string(), "Save error: Invalid index.");
        }

        let backup = rune2.id;
        rune2.id = 0;
        let result = rune2.change_effect(&mut file_data, 13101, 0);
        rune2.id = backup;
        assert!(result.is_err());
        if let Err(e) = result {
            assert_eq!(e.to_string(), "Save error: Failed to find the upgrade in the file data.");
        }

        //Change effects
        gem2.change_effect(&mut file_data, 13101, 1).unwrap();
        gem2.change_effect(&mut file_data, 14609, 2).unwrap();
        gem2.change_effect(&mut file_data, 14610, 3).unwrap();

        rune2.change_effect(&mut file_data, 1100000, 1).unwrap();
        rune2.change_effect(&mut file_data, 2107001, 2).unwrap();
        rune2.change_effect(&mut file_data, 2108001, 3).unwrap();

        //Compare all but effects
        let check = |upgrade_a: Upgrade, upgrade_b: Upgrade| -> bool {
            (upgrade_a.id == upgrade_b.id) &&
            (upgrade_a.source == upgrade_b.source) &&
            (upgrade_a.upgrade_type == upgrade_b.upgrade_type) &&
            (upgrade_a.shape == upgrade_b.shape) &&
            (upgrade_a.info == upgrade_b.info)
        };

        //Gem
        assert_eq!(gem2.effects, vec![(0x440c, String::from("Add physical ATK +45")),
                                         (13101, String::from("Add blood ATK +0.5")),
                                         (14609, String::from("Add arcane ATK +56.3")),
                                         (14610, String::from("Add arcane ATK +62.5")),
                                         (0x440c, String::from("Add physical ATK +45")),
                                         (0x440c, String::from("Add physical ATK +45"))]);
        assert!(check(gem.clone(), gem2.clone()));

        //Item N0
        assert_eq!(rune2.effects, vec![(0x115582, String::from("Max QS bullets held UP +3")),
                                          (1100000, String::from("More echoes from slain enemies 1")),
                                          (2107001, String::from("Vial HP recovery UP")),
                                          (2108001, String::from("Cont. heal near death +1")),
                                          (0xffffffff, String::from("No Effect")),
                                          (0xffffffff, String::from("No Effect"))]);
        assert!(check(rune.clone(), rune2.clone()));

        //Check the write to the file data
        let upgrades = parse_upgrades(&file_data);
        let gem3 = upgrades.get(&3229614145).unwrap().0.clone();
        let rune3 = upgrades.get(&3229614146).unwrap().0.clone();
        assert_eq!(gem2, gem3);
        assert_eq!(rune2, rune3);

        //TESTSAVE 0. Test Runes effects on gems and viceversa
        let mut file_data = FileData::build("saves/testsave0", PathBuf::from("resources")).unwrap();
        let upgrades = parse_upgrades(&file_data);
        let mut gem = upgrades.get(&3229614145).unwrap().0.clone();
        let mut rune = upgrades.get(&3229614146).unwrap().0.clone();

        //Change effects
        rune.change_effect(&mut file_data, 13101, 1).unwrap();
        rune.change_effect(&mut file_data, 14609, 2).unwrap();
        rune.change_effect(&mut file_data, 14610, 3).unwrap();

        gem.change_effect(&mut file_data, 1100000, 1).unwrap();
        gem.change_effect(&mut file_data, 2107001, 2).unwrap();
        gem.change_effect(&mut file_data, 2108001, 3).unwrap();

        //Gem
        assert_eq!(gem.effects, vec![(0x440c, String::from("Add physical ATK +45")),
                                         (1100000, String::from("More echoes from slain enemies 1")),
                                         (2107001, String::from("Vial HP recovery UP")),
                                         (2108001, String::from("Cont. heal near death +1")),
                                         (0x440c, String::from("Add physical ATK +45")),
                                         (0x440c, String::from("Add physical ATK +45"))]);

        //Item N0
        assert_eq!(rune.effects, vec![(0x115582, String::from("Max QS bullets held UP +3")),
                                         (13101, String::from("Add blood ATK +0.5")),
                                         (14609, String::from("Add arcane ATK +56.3")),
                                         (14610, String::from("Add arcane ATK +62.5")),
                                         (0xffffffff, String::from("No Effect")),
                                         (0xffffffff, String::from("No Effect"))]);

        //Check the write to the file data
        let upgrades = parse_upgrades(&file_data);
        let gem3 = upgrades.get(&3229614145).unwrap().0.clone();
        let rune3 = upgrades.get(&3229614146).unwrap().0.clone();
        assert_eq!(gem, gem3);
        assert_eq!(rune, rune3);

        //Test with value_index 0
        assert!(rune2.change_effect(&mut file_data, 2108001, 0).is_ok());
    }

    #[test]
    fn upgrade_transform() {
        //TESTSAVE 0
        let mut file_data = FileData::build("saves/testsave0", PathBuf::from("resources")).unwrap();
        let upgrades = parse_upgrades(&file_data);
        let mut gem = upgrades.get(&3229614145).unwrap().0.clone();
        let mut rune = upgrades.get(&3229614146).unwrap().0.clone();

        gem.transform(&mut file_data).unwrap(); //Transform the gem into a rune
        rune.transform(&mut file_data).unwrap();//Transform the rune into a gem
        //Rune -> Gem
        assert_eq!(rune.id, u32::from_le_bytes([0x42, 0x00, 0x80, 0xC0]));
        assert_eq!(rune.source, u32::from_le_bytes([0xBF, 0x92, 0x01, 0x80]));
        assert_eq!(rune.upgrade_type, UpgradeType::Gem);
        assert_eq!(rune.effects, vec![(13101, String::from("Add blood ATK +0.5")),
                                (0xFFFFFFFF, String::from("No Effect")),
                                (0xFFFFFFFF, String::from("No Effect")),
                                (0xFFFFFFFF, String::from("No Effect")),
                                (0xFFFFFFFF, String::from("No Effect")),
                                (0xFFFFFFFF, String::from("No Effect"))]);
        assert_eq!(rune.shape, String::from("Radial"));
        let info = rune.info.clone();
        assert_eq!(info.name, String::from("Bloodtinge Blood Gemstone (1)"));
        assert_eq!(info.effect, String::from("Add blood ATK +0.5"));
        assert_eq!(info.rating, 1);
        assert_eq!(info.level, 1);
        assert_eq!(info.note, String::from(""));

        //Gem -> Rune
        assert_eq!(gem.id, u32::from_le_bytes([0x41, 0x00, 0x80, 0xC0]));
        assert_eq!(gem.source, u32::from_le_bytes([0x26, 0x60, 0x01, 0x80]));
        assert_eq!(gem.upgrade_type, UpgradeType::Rune);
        assert_eq!(gem.effects, vec![(1100000, String::from("More echoes from slain enemies 1")),
                                (0xFFFFFFFF, String::from("No Effect")),
                                (0xFFFFFFFF, String::from("No Effect")),
                                (0xFFFFFFFF, String::from("No Effect")),
                                (0xFFFFFFFF, String::from("No Effect")),
                                (0xFFFFFFFF, String::from("No Effect"))]);
        assert_eq!(gem.shape, String::from("-"));
        let info = gem.info.clone();
        assert_eq!(info.name, String::from("Moon"));
        assert_eq!(info.effect, String::from("More echoes from slain enemies 1"));
        assert_eq!(info.rating, 0);
        assert_eq!(info.level, 0);
        assert_eq!(info.note, String::from("\"Moon\" rune. Acquire more Blood Echoes"));

        //Check if the file_data was modified correctly
        let upgrades = parse_upgrades(&file_data);
        let gem2 = upgrades.get(&u32::from_le_bytes([0x42, 0x00, 0x80, 0xC0])).unwrap().0.clone();
        let rune2 = upgrades.get(&u32::from_le_bytes([0x41, 0x00, 0x80, 0xC0])).unwrap().0.clone();
        assert_eq!(rune,gem2);
        assert_eq!(gem,rune2);

        gem.id = 0;
        let result = gem.transform(&mut file_data); //Transform the gem into a rune
        assert!(result.is_err());
        if let Err(e) = result {
            assert_eq!(e.to_string(), "Save error: Failed to find the upgrade in the file data.");
        }
    }

    #[test]
    fn test_get_shape() {
        let result = get_shape(255, UpgradeType::Gem);
        assert!(result.is_err());
        if let Err(e) = result {
            assert_eq!(e.to_string(), "Save error: Invalid shape number.");
        }

        let result = get_shape(255, UpgradeType::Rune);
        assert!(result.is_err());
        if let Err(e) = result {
            assert_eq!(e.to_string(), "Save error: Invalid shape number.");
        }
    }
}
