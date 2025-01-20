use serde::{Deserialize, Serialize};
use serde_json::{self, json, Value};
use super::{constants::*,
            enums::{ArticleType, Error, TypeFamily, UpgradeType},
            file::FileData,
            slots::Slot,
            upgrades::Upgrade,
            article::{Article, ItemInfo, WeaponMods, scale_weapon_info}};
use std::{fs::File,
          io::BufReader,
          collections::HashMap,
          path::PathBuf};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Inventory {
    pub articles: HashMap<ArticleType, Vec<Article>>,
    pub upgrades: HashMap<UpgradeType, Vec<Upgrade>>,
    //If the first slot of the inventory contains an article store its type
    pub first_article: Option<ArticleType>,
    //If it contains an upgrade store its type
    pub first_upgrade: Option<UpgradeType>,
}

impl Inventory {
    pub fn build(file_data: &FileData, inv: (usize, usize), key: (usize, usize), all_upgrades: &mut HashMap<u32, (Upgrade, UpgradeType)>, all_slots: &mut HashMap<u64, Vec<Slot>>) -> Inventory {
        let mut has_slots = false;
        let mut articles = HashMap::new();
        let mut upgrades = HashMap::new();
        let mut first_upgrade = None;
        let mut first_article = None;
        let mut first = true;

        let mut parse = |start: usize, end: usize| {
            for (index, i) in (start .. end).step_by(16).enumerate() {
                let number = file_data.bytes[i];
                let mut id = u32::from_le_bytes([file_data.bytes[i + 8], file_data.bytes[i + 9], file_data.bytes[i + 10], 0]);
                let first_part =
                    u32::from_le_bytes([file_data.bytes[i + 4], file_data.bytes[i + 5], file_data.bytes[i + 6], file_data.bytes[i + 7]]);
                let second_part =
                    u32::from_le_bytes([file_data.bytes[i + 8], file_data.bytes[i + 9], file_data.bytes[i + 10], file_data.bytes[i + 11]]);
                let amount =
                    u32::from_le_bytes([file_data.bytes[i + 12], file_data.bytes[i + 13], file_data.bytes[i + 14], file_data.bytes[i + 15]]);
    
                let result = match (file_data.bytes[i+7],file_data.bytes[i+11]) {
                    (0xB0,0x40) => {
                        has_slots = false;
                        get_info_item(id, &file_data.resources_path)
                    },
                    (_,0x10) => {
                        has_slots = true;
                        get_info_armor(id, &file_data.resources_path)
                    },
                    _ => {
                        id = second_part;
                        has_slots = true;
                        get_info_weapon(id, &file_data.resources_path)
                    },
                };
    
                if let Ok((info, article_type)) = result {
                    let mut slots = None;
                    if has_slots {
                        let key = u64::from_le_bytes([file_data.bytes[i+4],
                                                     file_data.bytes[i+5],
                                                     file_data.bytes[i+6],
                                                     file_data.bytes[i+7],
                                                     file_data.bytes[i+8],
                                                     file_data.bytes[i+9],
                                                     file_data.bytes[i+10],
                                                     file_data.bytes[i+11]]);
                        slots = all_slots.remove(&key);
                        if slots.is_none() {
                            continue;
                        }
                    }

                    let mut article = Article {
                        number,
                        id,
                        first_part,
                        second_part,
                        amount,
                        info,
                        article_type,
                        type_family: article_type.into(),
                        slots,
                        index,
                    };
                    if first {
                        first_article = Some(article_type);
                        first = false;
                    }
                    let category = articles.entry(article_type).or_insert(Vec::new());
                    article.index = category.len();
                    category.push(article);
                } else if let Some(mut upgrade) = all_upgrades.remove(&first_part){
                    if first {
                        first_upgrade = Some(upgrade.1);
                        first = false;
                    }
                    let category = upgrades.entry(upgrade.1).or_insert(Vec::new());
                    upgrade.0.index = category.len();
                    upgrade.0.number = number;
                    category.push(upgrade.0);
                };
            }
        };
        parse(inv.0, inv.1); //parse the inventory
        parse(key.0, key.1); //parse the key inventory
        Inventory {
            articles,
            upgrades,
            first_article,
            first_upgrade,
        }
    }
    ///Modifies the amount of an article
    pub fn edit_item(&mut self, file_data: &mut FileData, number: u8, id: u32, value: u32, is_storage: bool) -> Result<(), Error> {
        let value_endian = u32::to_le_bytes(value);
        let mut found = false;
        for (k, v) in self.articles.iter_mut() {
            let family: TypeFamily = k.to_owned().into();
            if family == TypeFamily::Item {
                if let Some(item) = v.iter_mut().find(|item| item.number == number) {
                    if k == &ArticleType::Key {
                        return Err(Error::CustomError("ERROR: Key items cannot be edited."));
                    }
                    item.amount = value;
                    found = true;
                    break;
                }
            }
        }

        let opt = file_data.find_article_offset(number, id, TypeFamily::Item, is_storage);
        if let Some(offset) = opt {
            for (i, o) in (offset+12 .. offset+16).enumerate() {
                file_data.bytes[o] = value_endian[i];
            }
        }

        if opt.is_none() || !found {
            return Err(Error::CustomError("ERROR: The Article was not found in the inventory."));
        }

        Ok(())
    }

    pub fn add_item(&mut self, file_data: &mut FileData, id: u32, quantity: u32, is_storage: bool) -> Result<&mut Inventory, Error> {
        let result = get_info_item(id, &file_data.resources_path);
        if result.is_err() {
            return Err(Error::CustomError("ERROR: failed to find info for the item."));
        }

        let (_, inventory_end) = {
            if !is_storage {
                file_data.offsets.inventory
            } else {
                file_data.offsets.storage
            }
        };
        let (first_counter_index, second_counter_index) = {
            if !is_storage {
                (USERNAME_TO_FIRST_INVENTORY_COUNTER, USERNAME_TO_SECOND_INVENTORY_COUNTER)
            } else {
                (USERNAME_TO_FIRST_STORAGE_COUNTER, USERNAME_TO_SECOND_STORAGE_COUNTER)
            }
        };

        let endian_id = u32::to_le_bytes(id);
        let endian_quantity = u32::to_le_bytes(quantity);

        for i in 0..12 {
            if i < 8 {
                file_data.bytes[inventory_end + i] = endian_id[i % 4];
            } else {
                file_data.bytes[inventory_end + i] = endian_quantity[i % 4];
            }
        }
        file_data.bytes[inventory_end + 3] = 0xB0;
        file_data.bytes[inventory_end + 7] = 0x40;
        (file_data.bytes[inventory_end + 12], _) = file_data.bytes[inventory_end - 4].overflowing_add(1);

        let id = u32::from_le_bytes(endian_id);

        // Create the first_part array
        let mut first_part = [0u8; 4];
        first_part[..endian_id.len()].copy_from_slice(&endian_id);
        first_part[first_part.len() - 1] = 0xB0;

        // Create the second_part array
        let mut second_part = [0u8; 4];
        second_part[..endian_id.len()].copy_from_slice(&endian_id);
        second_part[second_part.len() - 1] = 0x40;

        file_data.bytes[file_data.offsets.username + first_counter_index] += 1;
        file_data.bytes[file_data.offsets.username + second_counter_index] += 1;
        if !is_storage {
            file_data.offsets.inventory.1 += 16;
        } else {
            file_data.offsets.storage.1 += 16;
        }

        let (info, article_type) = result.expect("Err variant checked at the beginning");

        let mut new_item = Article {
            number: file_data.bytes[inventory_end - 4],
            id,
            first_part: u32::from_le_bytes(first_part),
            second_part: u32::from_le_bytes(second_part),
            info,
            amount: quantity,
            article_type,
            type_family: article_type.into(),
            slots: None,
            index: 0,
        };

        //Find the first item of the storage to increase it's index
        let mut found = false;
        if is_storage {
            if let Some(article_type) = self.first_article {
                if let Some(ref mut articles_of_type) = self.articles.get_mut(&article_type) {
                    if let Some(first) = articles_of_type.first_mut() {
                        first.number += 1;
                        found = true;
                    }
                }
            } else if let Some(upgrade_type) = self.first_upgrade {
                if let Some(ref mut upgrades_of_type) = self.upgrades.get_mut(&upgrade_type) {
                    if let Some(first) = upgrades_of_type.first_mut() {
                        first.number += 1;
                        found = true;
                    }
                }
            }
            if !found {
                new_item.number = file_data.bytes[file_data.offsets.username + first_counter_index];
            }
        }

        let vec = self.articles.entry(article_type).or_insert(Vec::new());
        new_item.index = vec.len();
        vec.push(new_item);

        return Ok(self);
    }

    //This method asumes that upgrade exists in file_data and it's not in self
    fn add_upgrade(&mut self, file_data: &mut FileData, mut upgrade: Upgrade, is_storage: bool) {
        let inventory_end = {
            if !is_storage {
                file_data.offsets.inventory.1
            } else {
                file_data.offsets.storage.1
            }
        };
        let (first_counter_offset, second_counter_offset) = {
            if !is_storage {
                (USERNAME_TO_FIRST_INVENTORY_COUNTER, USERNAME_TO_SECOND_INVENTORY_COUNTER)
            } else {
                (USERNAME_TO_FIRST_STORAGE_COUNTER, USERNAME_TO_SECOND_STORAGE_COUNTER)
            }
        };

        let endian_id = u32::to_le_bytes(upgrade.id);
        let endian_source = u32::to_le_bytes(upgrade.source);
        let endian_quantity = [0x01, 0x00, 0x00, 0x00];

        for i in 0..4 {
            file_data.bytes[inventory_end + i] = endian_id[i];
        }
        for i in 4..8 {
            file_data.bytes[inventory_end + i] = endian_source[i % 4];
        }
        for i in 8..12 {
            file_data.bytes[inventory_end + i] = endian_quantity[i % 4];
        }

        (file_data.bytes[inventory_end + 12], _) = file_data.bytes[inventory_end - 4].overflowing_add(1);

        file_data.bytes[file_data.offsets.username + first_counter_offset] += 1;
        file_data.bytes[file_data.offsets.username + second_counter_offset] += 1;
        if !is_storage {
            file_data.offsets.inventory.1 += 16;
        } else {
            file_data.offsets.storage.1 += 16;
        }

        //Find the first item of the storage to increase it's index
        let mut found = false;
        if is_storage {
            if let Some(article_type) = self.first_article {
                if let Some(ref mut articles_of_type) = self.articles.get_mut(&article_type) {
                    if let Some(first) = articles_of_type.first_mut() {
                        first.number += 1;
                        found = true;
                    }
                }
            } else if let Some(upgrade_type) = self.first_upgrade {
                if let Some(ref mut upgrades_of_type) = self.upgrades.get_mut(&upgrade_type) {
                    if let Some(first) = upgrades_of_type.first_mut() {
                        first.number += 1;
                        found = true;
                    }
                }
            }
            if !found {
                upgrade.number = file_data.bytes[file_data.offsets.username + first_counter_offset];
            }
        }

        let vec = self.upgrades.entry(upgrade.upgrade_type).or_insert(Vec::new());
        upgrade.index = vec.len();
        vec.push(upgrade);
    }
}


pub fn get_info_item(id: u32, resources_path: &PathBuf) -> Result<(ItemInfo, ArticleType), Error> {
    let file_path = resources_path.join("items.json");
    let json_file =  File::open(file_path).map_err(Error::IoError)?;
    let reader = BufReader::new(json_file);
    let items: Value = serde_json::from_reader(reader).unwrap();
    let items = items.as_object().unwrap();

    for (category, category_items) in items {
        match category_items.as_object().unwrap().keys().find(|x| x.parse::<u32>().unwrap() == id) {
            Some(found) => {
                let mut info: ItemInfo = serde_json::from_value(category_items[found].clone()).unwrap(); 
                if category == "chalice" {
                    info.extra_info = Some(json!({
                        "depth": &category_items[found]["depth"],
                        "area": &category_items[found]["area"],
                    }));
                }
                return Ok((info, ArticleType::from(category.as_str())))
            },
            None => ()
        }
    }
    Err(Error::CustomError("ERROR: Failed to find info for the item."))
}

pub fn get_info_armor(id: u32, resources_path: &PathBuf) -> Result<(ItemInfo, ArticleType), Error> {
    let file_path = resources_path.join("armors.json");
    let json_file =  File::open(file_path).map_err(Error::IoError)?;
    let reader = BufReader::new(json_file);
    let armors: Value = serde_json::from_reader(reader).unwrap();
    let armors = armors.as_object().unwrap();

    match armors.keys().find(|x| x.parse::<u32>().unwrap() == id) {
        Some(found) => {
            let mut info: ItemInfo = serde_json::from_value(armors[found].clone()).unwrap();
            info.extra_info = Some(json!({
                "physicalDefense": &armors[found]["physicalDefense"],
                "elementalDefense": &armors[found]["elementalDefense"],
                "resistance": &armors[found]["resistance"],
                "beasthood": &armors[found]["beasthood"]
            }));
            return Ok((info, ArticleType::Armor))
        },
        None => ()
    }
    Err(Error::CustomError("ERROR: Failed to find info for the armor."))
}

pub fn get_info_weapon(mut id: u32, resources_path: &PathBuf) -> Result<(ItemInfo, ArticleType), Error> {
    let file_path = resources_path.join("weapons.json");
    let json_file =  File::open(file_path).map_err(Error::IoError)?;
    let reader = BufReader::new(json_file);
    let weapons: Value = serde_json::from_reader(reader).unwrap();
    let weapons = weapons.as_object().unwrap();

    let weapon_mods = WeaponMods::try_from(id)?;
    if id != 12080000 && id != 6180000 { //Special case
        id = (id / 100000) * 100000; //Remove the weapon mods to be able to find its info
    }
    for (category, category_weapons) in weapons {
        match category_weapons.as_object().unwrap().keys().find(|x| x.parse::<u32>().unwrap() == id) {
            Some(found) => {
                let mut info: ItemInfo = serde_json::from_value(category_weapons[found].clone()).unwrap();
                let mut extra_info = json!({
                    "damage": &category_weapons[found]["damage"],
                    "upgrade_level": weapon_mods.upgrade_level,
                    "imprint": weapon_mods.imprint,
                });
                if weapon_mods.upgrade_level > 0 {
                    scale_weapon_info(&mut extra_info);
                }
                info.extra_info = Some(extra_info);
                return Ok((info, ArticleType::from(category.as_str())))
            },
            None => ()
        }
    }
    Err(Error::CustomError("ERROR: Failed to find info for the weapon."))
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::data_handling::{upgrades::parse_upgrades,
                              enums::SlotShape,
                              utils::test_utils::{check_bytes, build_save_data},
                              slots::parse_equipped_gems};

    #[test]
    fn inventory_edit_item() {
        let mut save = build_save_data("testsave0");
        assert!(check_bytes(&save.file, 0x89cc, 
            &[0x48,0x80,0xCF,0xA8,0x64,0,0,0xB0,0x64,0,0,0x40,0x01,0,0,0]));
        //Try to edit a key item
        let result = save.inventory.edit_item(&mut save.file, 0x00, 0xAAAAAAAA, 0xAABBCCDD, false);
        assert!(result.is_err());
        if let Err(error) = result {
            assert_eq!(error.to_string(), "Save error: ERROR: Key items cannot be edited.");
        }
        //Try wrong index
        let result = save.inventory.edit_item(&mut save.file, 0xAA, 0xAAAAAAAA, 0xAABBCCDD, false);
        assert!(result.is_err());
        if let Err(error) = result {
            assert_eq!(error.to_string(), "Save error: ERROR: The Article was not found in the inventory.");
        }

        save.inventory.edit_item(&mut save.file, 0x48, 0x64, 0xAABBCCDD, false).unwrap();
        assert!(check_bytes(&save.file, 0x89cc, 
            &[0x48,0x80,0xCF,0xA8,0x64,0,0,0xB0,0x64,0,0,0x40,0xDD,0xCC,0xBB,0xAA]));
        assert_eq!(save.inventory.articles.get(&ArticleType::Consumable).unwrap()[0].amount, 0xAABBCCDD);
    }

    #[test]
    fn test_parse_key_inventory() {
        let save = build_save_data("testsave0");
        let keys = save.inventory.articles.get(&ArticleType::Key).unwrap();
        assert_eq!(keys.len(), 7);

        //Item N0
        assert_eq!(keys[0].number, 107);
        assert_eq!(keys[0].id, u32::from_le_bytes([0xa9, 0x0f, 0x00, 0x00]));
        assert_eq!(keys[0].first_part, u32::from_le_bytes([0xa9, 0x0f, 0x00, 0xb0]));
        assert_eq!(keys[0].second_part, u32::from_le_bytes([0xa9, 0x0f, 0x00, 0x40]));
        assert_eq!(keys[0].amount, u32::from_le_bytes([0x01, 0x00, 0x00, 0x00]));
        assert_eq!(keys[0].article_type, ArticleType::Key);

        //Item N1
        assert_eq!(keys[1].number, 6);
        assert_eq!(keys[1].id, u32::from_le_bytes([0x12, 0x10, 0x00, 0x00]));
        assert_eq!(keys[1].first_part, u32::from_le_bytes([0x12, 0x10, 0x00, 0xb0]));
        assert_eq!(keys[1].second_part, u32::from_le_bytes([0x12, 0x10, 0x00, 0x40]));
        assert_eq!(keys[1].amount, u32::from_le_bytes([0x01, 0x00, 0x00, 0x00]));
        assert_eq!(keys[1].article_type, ArticleType::Key);

        //Item N2
        assert_eq!(keys[2].number, 0);
        assert_eq!(keys[2].id, u32::from_le_bytes([0xd8, 0x10, 0x00, 0x00]));
        assert_eq!(keys[2].first_part, u32::from_le_bytes([0xd8, 0x10, 0x00, 0xb0]));
        assert_eq!(keys[2].second_part, u32::from_le_bytes([0xd8, 0x10, 0x00, 0x40]));
        assert_eq!(keys[2].amount, u32::from_le_bytes([0x01, 0x00, 0x00, 0x00]));
        assert_eq!(keys[2].article_type, ArticleType::Key);

        //Item N3
        assert_eq!(keys[3].number, 1);
        assert_eq!(keys[3].id, u32::from_le_bytes([0x0e, 0x10, 0x00, 0x00]));
        assert_eq!(keys[3].first_part, u32::from_le_bytes([0x0e, 0x10, 0x00, 0xb0]));
        assert_eq!(keys[3].second_part, u32::from_le_bytes([0x0e, 0x10, 0x00, 0x40]));
        assert_eq!(keys[3].amount, u32::from_le_bytes([0x01, 0x00, 0x00, 0x00]));
        assert_eq!(keys[3].article_type, ArticleType::Key);

        //Item N4
        assert_eq!(keys[4].number, 2);
        assert_eq!(keys[4].id, u32::from_le_bytes([0xa0, 0x0f, 0x00, 0x00]));
        assert_eq!(keys[4].first_part, u32::from_le_bytes([0xa0, 0x0f, 0x00, 0xb0]));
        assert_eq!(keys[4].second_part, u32::from_le_bytes([0xa0, 0x0f, 0x00, 0x40]));
        assert_eq!(keys[4].amount, u32::from_le_bytes([0x01, 0x00, 0x00, 0x00]));
        assert_eq!(keys[4].article_type, ArticleType::Key);

        //Item N5
        assert_eq!(keys[5].number, 3);
        assert_eq!(keys[5].id, u32::from_le_bytes([0x07, 0x10, 0x00, 0x00]));
        assert_eq!(keys[5].first_part, u32::from_le_bytes([0x07, 0x10, 0x00, 0xb0]));
        assert_eq!(keys[5].second_part, u32::from_le_bytes([0x07, 0x10, 0x00, 0x40]));
        assert_eq!(keys[5].amount, u32::from_le_bytes([0x01, 0x00, 0x00, 0x00]));
        assert_eq!(keys[5].article_type, ArticleType::Key);

        //Item N6
        assert_eq!(keys[6].number, 4);
        assert_eq!(keys[6].id, u32::from_le_bytes([0xab, 0x0f, 0x00, 0x00]));
        assert_eq!(keys[6].first_part, u32::from_le_bytes([0xab, 0x0f, 0x00, 0xb0]));
        assert_eq!(keys[6].second_part, u32::from_le_bytes([0xab, 0x0f, 0x00, 0x40]));
        assert_eq!(keys[6].amount, u32::from_le_bytes([0x01, 0x00, 0x00, 0x00]));
        assert_eq!(keys[6].article_type, ArticleType::Key);
    }

    #[test]
    fn inventory_add_item() {
        let mut save = build_save_data("testsave0");
        assert_eq!(save.inventory.articles.get(&ArticleType::Consumable).unwrap().len(), 17);
        assert!(check_bytes(&save.file, 0x8ccc,
            &[0x78,0xff,0xff,0xff,0x00,0x00,0x00,0x00,0xff,0xff,0xff,0xff,0x00,0x00,0x00,0x00]));
        //Try to add an invalid item
        let result = save.inventory.add_item(&mut save.file, 0x00, 0x00, false);
        assert!(result.is_err());
        if let Err(error) = result {
            assert_eq!(error.to_string(), "Save error: ERROR: failed to find info for the item.");
        }

        //Add to the storage
        assert!(save.inventory.add_item(&mut save.file, u32::from_le_bytes([0x60, 0x04, 0x00, 0x00]), 32, true).is_ok());

        //Add to the inventory
        save.inventory.add_item(&mut save.file, u32::from_le_bytes([0x60, 0x04, 0x00, 0x00]), 32, false).unwrap();
        assert_eq!(save.inventory.articles.get(&ArticleType::Consumable).unwrap().len(), 19);
        assert!(check_bytes(&save.file, 0x8ccc,
            &[0x78,0xff,0xff,0xff,0x60,0x04,0x00,0xb0,0x60,0x04,0x00,0x40,0x20,0x00,0x00,0x00,
              0x79,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0xff,0xff,0xff,0xff,0x00,0x00,0x00,0x00]));


        let mut upgrades = parse_upgrades(&save.file);
        let mut slots = parse_equipped_gems(&mut save.file, &mut upgrades);
        let inventory = Inventory::build(&save.file, save.file.offsets.inventory, save.file.offsets.key_inventory, &mut upgrades, &mut slots);
        let consumables = inventory.articles.get(&ArticleType::Consumable).unwrap();
        let new_item = consumables.last().unwrap();
        assert_eq!(new_item.number, 120);
        assert_eq!(new_item.id, u32::from_le_bytes([0x60, 0x04, 0x00, 0x00]));
        assert_eq!(new_item.first_part, u32::from_le_bytes([0x60, 0x04, 0x00, 0xb0]));
        assert_eq!(new_item.second_part, u32::from_le_bytes([0x60, 0x04, 0x00, 0x40]));
        assert_eq!(new_item.amount, u32::from_le_bytes([0x20, 0x00, 0x00, 0x00]));
        assert_eq!(new_item.article_type, ArticleType::Consumable);

        //Add to a save without items in its storage
        let mut save = build_save_data("testsave7");
        save.storage.add_item(&mut save.file, u32::from_le_bytes([0x60, 0x04, 0x00, 0x00]), 32, true).unwrap();
        let consumables = save.storage.articles.get(&ArticleType::Consumable).unwrap();
        let new_item = consumables.last().unwrap();
        assert_eq!(new_item.number, 1);
        assert_eq!(new_item.id, u32::from_le_bytes([0x60, 0x04, 0x00, 0x00]));
        assert_eq!(new_item.first_part, u32::from_le_bytes([0x60, 0x04, 0x00, 0xb0]));
        assert_eq!(new_item.second_part, u32::from_le_bytes([0x60, 0x04, 0x00, 0x40]));
        assert_eq!(new_item.amount, u32::from_le_bytes([0x20, 0x00, 0x00, 0x00]));
        assert_eq!(new_item.article_type, ArticleType::Consumable);

    }

    #[test]
    fn test_equipped_gems() {
        //This tests if Inventory::build allocates gems correctly
        //(To the player inventory, storage, and weapon slots)
        let save = build_save_data("testsave9");

        //Inventory:
        let gems = save.inventory.upgrades.get(&UpgradeType::Gem).unwrap();
        let runes = save.inventory.upgrades.get(&UpgradeType::Rune).unwrap();
        assert_eq!(runes[0].id, u32::from_le_bytes([0x72, 0x00, 0x80, 0xC0]));
        assert_eq!(runes[0].source, u32::from_le_bytes([0x4a, 0x0d, 0x03, 0x80]));

        //This rune is equipped
        //assert_eq!(runes[1].id, u32::from_le_bytes([0x76, 0x00, 0x80, 0xC0]));
        //assert_eq!(runes[1].source, u32::from_le_bytes([0x40, 0x0d, 0x03, 0x80]));

        //This rune is equipped
        //assert_eq!(runes[2].id, u32::from_le_bytes([0x77, 0x00, 0x80, 0xC0]));
        //assert_eq!(runes[2].source, u32::from_le_bytes([0xbf, 0x92, 0x01, 0x80]));

        assert_eq!(gems[0].id, u32::from_le_bytes([0x73, 0x00, 0x80, 0xC0]));
        assert_eq!(gems[0].source, u32::from_le_bytes([0xf0, 0x49, 0x02, 0x80]));

        assert_eq!(gems[1].id, u32::from_le_bytes([0x70, 0x00, 0x80, 0xC0]));
        assert_eq!(gems[1].source, u32::from_le_bytes([0xf2, 0x49, 0x02, 0x80]));


        //Storage:
        let gems = save.storage.upgrades.get(&UpgradeType::Gem).unwrap();

        assert_eq!(gems[0].id, u32::from_le_bytes([0x71, 0x00, 0x80, 0xC0]));
        assert_eq!(gems[0].source, u32::from_le_bytes([0x48, 0xe8, 0x01, 0x80]));

        assert_eq!(gems[1].id, u32::from_le_bytes([0x75, 0x00, 0x80, 0xC0]));
        assert_eq!(gems[1].source, u32::from_le_bytes([0x1a, 0xf0, 0x01, 0x80]));

        //Equipped gems
        let weapons = save.inventory.articles.get(&ArticleType::RightHand).unwrap();
        assert_eq!(weapons[0].id, 5000300);
        let slots = weapons[0].slots.clone().unwrap();

        //Slot 1
        assert_eq!(slots[0].shape, SlotShape::Radial);
        let gem = slots[0].gem.clone().unwrap();
        assert_eq!(gem.id, u32::from_le_bytes([0x74, 0x00, 0x80, 0xc0]));
        assert_eq!(gem.shape, String::from("Radial"));
        let info = gem.info.clone();
        assert_eq!(info.name, String::from("Tempering Blood Gemstone (2)"));
        assert_eq!(info.effect, String::from("Physical ATK UP +7.3%"));
        assert_eq!(info.rating, 7);
        assert_eq!(info.level, 2);
        assert_eq!(info.note, String::from(""));

        //Slot 2
        assert_eq!(slots[1].shape, SlotShape::Radial);
        let gem = slots[1].gem.clone().unwrap();
        assert_eq!(gem.id, u32::from_le_bytes([0x6f, 0x00, 0x80, 0xc0]));
        assert_eq!(gem.shape, String::from("Radial"));
        let info = gem.info.clone();
        assert_eq!(info.name, String::from("Tempering Blood Gemstone (3)"));
        assert_eq!(info.effect, String::from("Physical ATK UP +9.5%"));
        assert_eq!(info.rating, 9);
        assert_eq!(info.level, 3);
        assert_eq!(info.note, String::from(""));

        //Slot 3
        assert_eq!(slots[2].shape, SlotShape::Closed);
        assert!(slots[2].gem.is_none());

        //Slot 4
        assert_eq!(slots[3].shape, SlotShape::Closed);
        assert!(slots[3].gem.is_none());

        //Slot 5
        assert_eq!(slots[4].shape, SlotShape::Closed);
        assert!(slots[4].gem.is_none());
    }

    #[test]
    fn inventory_add_upgrade() {
        let mut save = build_save_data("testsave0");
        let runes = save.inventory.upgrades.get(&UpgradeType::Rune).unwrap();
        let rune = runes[0].clone();
        assert_eq!(runes.len(), 1);
        assert!(check_bytes(&save.file, 0x8ccc,
            &[0x78,0xff,0xff,0xff,0x00,0x00,0x00,0x00,0xff,0xff,0xff,0xff,0x00,0x00,0x00,0x00]));


        //Add to the inventory
        save.inventory.add_upgrade(&mut save.file, rune.clone(), false);
        let runes = save.inventory.upgrades.get(&UpgradeType::Rune).unwrap();
        let mut rune2 = runes[1].clone();
        rune2.index = 0;
        assert_eq!(rune, rune2);
        assert_eq!(runes.len(), 2);
        assert!(check_bytes(&save.file, 0x8ccc,
            &[0x78,0xff,0xff,0xff,0x42,0x00,0x80,0xc0,0xbf,0x92,0x01,0x80,0x01,0x00,0x00,0x00,
              0x79,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0xff,0xff,0xff,0xff,0x00,0x00,0x00,0x00]));

        //Add to a save without items in its storage
        let mut save = build_save_data("testsave7");
        save.storage.add_upgrade(&mut save.file, rune, true);
        let runes = save.storage.upgrades.get(&UpgradeType::Rune).unwrap();
        let new_rune = runes.last().unwrap();
        assert_eq!(new_rune.number, 1);
        assert_eq!(new_rune.id, u32::from_le_bytes([0x42,0x00,0x80,0xc0]));
    }
}
