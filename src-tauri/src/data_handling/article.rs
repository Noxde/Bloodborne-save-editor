use serde::{Deserialize, Serialize};
use serde_json::{self, json, Value};
use super::{enums::{ArticleType, Error, Imprint, TypeFamily, SlotShape},
            file::FileData,
            slots::Slot,
            inventory::{get_info_item, get_info_armor, get_info_weapon}};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ItemInfo {
    pub item_name: String,
    pub item_desc: String,
    pub item_img: String,
    pub extra_info: Option<Value>,
}

//Describes the imprint and the upgrade level of a weapon
pub struct WeaponMods {
    pub upgrade_level: u8,
    pub imprint: Option<Imprint>,
}

impl TryFrom<u32> for WeaponMods {
    type Error = Error;
    fn try_from(second_part: u32) -> Result<Self, Self::Error> {
        let substract = second_part % 10000;
        let upgrade_level = (substract / 100) as u8;
        let imprint = match (second_part % 100000) - substract {
            0 | 80000 => None,
            10000 => Some(Imprint::Uncanny),
            20000 => Some(Imprint::Lost),
            _ => return Err(Error::CustomError("ERROR: Invalid second_part")),
        };
        Ok(WeaponMods {
            upgrade_level,
            imprint,
        })
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Article {
    pub number: u8, //First byte of the inventory slot
    pub id: u32,
    pub first_part: u32,
    pub second_part: u32,
    pub amount: u32,
    pub info: ItemInfo,
    pub article_type: ArticleType,
    pub type_family: TypeFamily,
    pub slots: Option<Vec<Slot>>,
    pub index: usize, //Index of the article inside the vector
}

impl Article {
    pub fn transform(&mut self, file_data: &mut FileData, new_id: u32, is_storage: bool) -> Result<(), Error>{
        let mut new_id = new_id.to_le_bytes().to_vec();
        match self.type_family {
            TypeFamily::Item => {
                new_id.pop();
                self.transform_item(file_data, new_id, is_storage)
            },
            TypeFamily::Armor | TypeFamily::Weapon => self.transform_armor_or_weapon(file_data, new_id, is_storage),
        }
    }
    fn transform_item(&mut self, file_data: &mut FileData, new_id: Vec<u8>, is_storage: bool) -> Result<(), Error>{
        if new_id.len()!=3 {
            return Err(Error::CustomError("ERROR: 'new_id' argument must be 3B long."));
        }

        //ID
        let id = u32::from_le_bytes([new_id[0],new_id[1],new_id[2],0]);

        let info;
        let article_type;
        let type_family;
        //INFO & ARTICLE_TYPE
        if let Ok((new_info, new_article_type)) = get_info_item(id, &file_data.resources_path) {
            info = new_info;
            article_type = new_article_type;
            type_family = article_type.into();
        } else {
            return Err(Error::CustomError("ERROR: Failed to find info for the item."));
        }

        let i;
        match file_data.find_article_offset(self.number, self.id, self.type_family, is_storage) {
            Some(offset) => i = offset,
            None => return Err(Error::CustomError("ERROR: The Article was not found in the inventory.")),
        }

        //Only update data if the item is valid
        //FIRST PART
        for j in i+4..=i+6 {
            file_data.bytes[j] = new_id[j-i-4];
        }
        let first_part = u32::from_le_bytes([new_id[0],new_id[1],new_id[2],0xB0]);

        //SECOND PART
        for j in i+8..=i+10 {
            file_data.bytes[j] = new_id[j-i-8];
        }
        let second_part = u32::from_le_bytes([new_id[0],new_id[1],new_id[2],0x40]);

        //Set amount to 1 if the item is a key or a chalice
        if (article_type == ArticleType::Key) || (article_type == ArticleType::Chalice) {
            let amount = vec![0x01, 0x00, 0x00, 0x00];
            for j in i+12..=i+15 {
                file_data.bytes[j] = amount[j-i-12];
            }
            self.amount = 1;
        }

        self.first_part = first_part;
        self.second_part = second_part;
        self.info = info;
        self.article_type = article_type;
        self.type_family = type_family;
        self.id = id;
        Ok(())
    }

    fn transform_armor_or_weapon(&mut self, file_data: &mut FileData, new_id: Vec<u8>, is_storage: bool) -> Result<(), Error>{
        if new_id.len()!=4 {
            return Err(Error::CustomError("ERROR: 'new_id' argument must be 4B long."));
        }

        let i;
        match file_data.find_article_offset(self.number, self.id, self.type_family, is_storage) {
            Some(offset) => i = offset,
            None => return Err(Error::CustomError("ERROR: The Article was not found in the inventory.")),
        }
        //Take the first and second part to search later
        let mut query = Vec::with_capacity(8);
        let mut byte_count = 4;

        for j in 4..=11 {
            query.push(file_data.bytes[i+j]);
        }

        //ID
        let id = u32::from_le_bytes([new_id[0],new_id[1],new_id[2],new_id[3]]);
        let result;
        let second_part;
        let info;
        let article_type;
        let type_family;

        if self.article_type == ArticleType::Armor {
            second_part = u32::from_le_bytes([new_id[0],new_id[1],new_id[2],0x10]);
            byte_count = 3;
            result = get_info_armor(id, &file_data.resources_path);
        } else {
            second_part = u32::from_le_bytes([new_id[0],new_id[1],new_id[2],new_id[3]]);
            result = get_info_weapon(id, &file_data.resources_path);
        }

        //INFO & ARTICLE_TYPE
        if let Ok((new_info, new_article_type)) = result {
            info = new_info;
            article_type = new_article_type;
            type_family = article_type.into();
        } else {
            return Err(Error::CustomError("ERROR: Failed to find info for the article."));
        }
        //Update data only if the article is valid

        //SECOND PART
        for j in i+8..i+8+byte_count {
            file_data.bytes[j] = new_id[j-i-8];
        }

        //Search for the query above the inventory (where the article appears with its gems)
        let mut found = false;
        let mut index = 0;
        for j in (0..(file_data.offsets.inventory.0 - 8)).rev() {
            if query == file_data.bytes[j..=(j + 7)] {
                found = true;
                index = j;
                break;
            }
        }

        if !found {
            return Err(Error::CustomError("ERROR: The Article was not found above the inventory."))
        } else {
            //Update the article id
            for j in index+4..index+4+byte_count {
                file_data.bytes[j] = new_id[j-index-4];
            }
        }

        self.id = id;
        self.info = info;
        self.second_part = second_part;
        self.article_type = article_type;
        self.type_family = type_family;
        return Ok(())
    }

    pub fn is_armor(&self) -> bool {
        self.type_family == TypeFamily::Armor
    }

    pub fn is_item(&self) -> bool {
        self.type_family == TypeFamily::Item
    }

    pub fn is_weapon(&self) -> bool {
        self.type_family == TypeFamily::Weapon
    }

    pub fn set_imprint_and_upgrade(&mut self, file_data: &mut FileData, imprint: Option<Option<Imprint>>, upgrade_level: Option<u8>) -> Result<(), Error> {
        if !self.is_weapon() {
            return Err(Error::CustomError("ERROR: The article must be a weapon."));
        }
        let mut weapon_mods = match WeaponMods::try_from(self.second_part) {
            Ok(wm) => wm,
            Err(_) => return Err(Error::CustomError("ERROR: Invalid second_part")),
        };

        let mut new_second_part = (self.second_part / 100000) * 100000;
        if let Some(extra_info) = &mut self.info.extra_info {
            if let Some(imp) = imprint {
                extra_info["imprint"] = json!(imp);
                weapon_mods.imprint = imp;
            }
            if let Some(upg) = upgrade_level {
                if upg > 10 {
                    return Err(Error::CustomError("ERROR: Upgrade level cannot be bigger than 10."));
                }
                extra_info["upgrade_level"] = json!(upg);
                weapon_mods.upgrade_level = upg;
            }
            scale_weapon_info(extra_info);
        } else {
            return Err(Error::CustomError("ERROR: The weapon has no extrainfo."));
        }
        new_second_part += match weapon_mods.imprint {
            None => 0,
            Some(Imprint::Uncanny) => 10000,
            Some(Imprint::Lost) => 20000,
        };
        new_second_part += 100 * (weapon_mods.upgrade_level as u32);
        let new_second_part_array = new_second_part.to_le_bytes();
        let second_part_array = self.second_part.to_le_bytes();

        //Update the second part in the inventory
        let mut index;
        match file_data.find_article_offset(self.number, self.id, self.type_family, false) {
            Some(offset) => index = offset,
            None => return Err(Error::CustomError("ERROR: The Article was not found in the inventory.")),
        }
        for i in index+8 ..= index+11 {
            file_data.bytes[i] = new_second_part_array[i-index-8];
        }

        //Update the second part above the inventory
        let mut found = false;
        for i in (0..(file_data.offsets.inventory.0 - 8)).rev() {
            if second_part_array == file_data.bytes[i+8 ..= i+11] {
                found = true;
                index = i;
                for i in index+8 ..= index+11 {
                    file_data.bytes[i] = new_second_part_array[i-index-8];
                }
                break;
            }
        }
        if !found {
            return Err(Error::CustomError("ERROR: The weapon was not found above the inventory."))
        }

        self.id = new_second_part;
        self.second_part = new_second_part;
        Ok(())
    }

    pub fn change_slot_shape(&mut self, file_data: &mut FileData, slot_index: usize, new_shape: SlotShape) -> Result<(), Error> {
        if let Some(slots) = &mut self.slots {
            if let Some(slot) = slots.get_mut(slot_index) {
                let first_part = self.first_part.to_le_bytes();
                let second_part = self.second_part.to_le_bytes();
                let mut found = false;
                for i in file_data.offsets.equipped_gems.0 .. file_data.offsets.equipped_gems.1 {
                    if (file_data.bytes[i .. i+4] == first_part) && (file_data.bytes[i+4 .. i+8] == second_part) {
                        found = true;
                        let new_shape_bytes: [u8; 4] = new_shape.into();
                        //20 is the index for the shape of the first slot
                        let slot_index = i + 20 + 8 * slot_index;
                        for (j, offset) in (slot_index .. slot_index + 4).enumerate() {
                            file_data.bytes[offset] = new_shape_bytes[j];
                        }
                    }
                }

                if !found {
                    Err(Error::CustomError("ERROR: Failed to find the article in the file data."))
                } else {
                    slot.shape = new_shape;
                    Ok(())
                }
            } else {
                Err(Error::CustomError("ERROR: Invalid slot_index."))
            }
        } else {
                Err(Error::CustomError("ERROR: Article does not have slots."))
        }
    }
}

pub fn scale_weapon_info(extra_info: &mut Value) {
    //Scales the stats of a weapon based on its upgrade level
    let upgrade_level: u32 = serde_json::from_value(extra_info["upgrade_level"].clone()).unwrap();
    for (_, v) in extra_info["damage"].as_object_mut().unwrap() {
        //Get the damage for the current type
        let damage: String = serde_json::from_value(v.clone()).unwrap();
        let mut damage: u32 = match damage.parse::<u32>() {
            Ok(num) => num,
            Err(_) => continue, //If the damage is "-" (n/a), skip
        };

        if upgrade_level == 10 {
            damage *= 2;
        } else {
            damage += (damage / 10) * upgrade_level;
        }
        *v = json!(damage.to_string());
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data_handling::utils::test_utils::{build_save_data, check_bytes, build_file_data};

    #[test]
    fn article_transform_item() {
        let mut save = build_save_data("testsave0");

        let article = &mut save.inventory.articles.get_mut(&ArticleType::Consumable).unwrap()[0];
        assert!(check_bytes(&save.file, 0x89cc,
            &[0x48,0x80,0xCF,0xA8,0x64,0,0,0xB0,0x64,0,0,0x40,0x01,0,0,0]));
        let result = article.transform(&mut save.file, u32::from_le_bytes([0xAA,0xBB,0xCC,0x00]), false);
        assert!(result.is_err());
        if let Err(error) = result {
            assert_eq!(error.to_string(), "Save error: ERROR: Failed to find info for the item.");
        }
        article.transform(&mut save.file, u32::from_le_bytes([0x64,0x1B,0x00,0x00]), false).unwrap();
        assert!(check_bytes(&save.file, 0x89cc,
            &[0x48,0x80,0xCF,0xA8,0x64,0x1B,0x00,0xB0,0x64,0x1B,0x00,0x40,0x01,0,0,0]));
        assert_eq!(article.id, u32::from_le_bytes([0x64,0x1B,0x00,0x00]));
        assert_eq!(article.first_part, u32::from_le_bytes([0x64,0x1B,0x00,0xB0]));
        assert_eq!(article.second_part, u32::from_le_bytes([0x64,0x1B,0x00,0x40]));
        assert_eq!(article.article_type, ArticleType::Material);

        //error tests
        assert!(article.transform_item(&mut save.file, vec![0xAA,0xBB], false).is_err());
        assert!(article.transform(&mut save.file, u32::from_le_bytes([0xAA,0xBB,0xCC,0xDD]), false).is_err());
        article.number = 255;
        let result = article.transform(&mut save.file, u32::from_le_bytes([0x64,0x1B,0x00,0x00]), false);
        assert!(result.is_err());
        if let Err(error) = result {
            assert_eq!(error.to_string(), "Save error: ERROR: The Article was not found in the inventory.");
        }

        //test transforming a Consumable into a Key
        let article = &mut save.inventory.articles.get_mut(&ArticleType::Consumable).unwrap()[1];
        assert_eq!(article.amount, 20);
        assert!(check_bytes(&save.file, 0x89FC,
            &[0x4B,0x00,0xFD,0x7F,0x84,0x03,0x00,0xB0,0x84,0x03,0x00,0x40,0x14,0x00,0x00,0x00]));
        article.transform(&mut save.file, u32::from_le_bytes([0xA0,0x0F,0x00,0x00]), false).unwrap();
        assert!(check_bytes(&save.file, 0x89FC,
            &[0x4B,0x00,0xFD,0x7F,0xA0,0x0F,0x00,0xB0,0xA0,0x0F,0x00,0x40,0x01,0x00,0x00,0x00]));
        assert_eq!(article.article_type, ArticleType::Key);
        assert_eq!(article.amount, 1);

        //Transform an item in the key inventory
        let article = &mut save.inventory.articles.get_mut(&ArticleType::Key).unwrap()[2];
        assert_eq!(article.id, u32::from_le_bytes([0xd8, 0x10, 0x00,0x00]));
        assert_eq!(article.amount, 1);
        assert!(check_bytes(&save.file, 0x10550,
            &[0x00,0xc0,0x07,0xa6,0xd8,0x10,0x00,0xb0,0xd8,0x10,0x00,0x40,0x01,0x00,0x00,0x00]));
        article.transform(&mut save.file, u32::from_le_bytes([0xea,0x10,0x00,0x00]), false).unwrap();
        assert!(check_bytes(&save.file, 0x10550,
            &[0x00,0xc0,0x07,0xa6,0xea,0x10,0x00,0xb0,0xea,0x10,0x00,0x40,0x01,0x00,0x00,0x00]));
        assert_eq!(article.article_type, ArticleType::Key);
        assert_eq!(article.amount, 1);
    }

    #[test]
    fn article_transform_armor_or_weapon() {
        //Try to transform a weapon whose first occurence was deleted
        let mut save = build_save_data("testsave0");
        let article = &mut save.inventory.articles.get_mut(&ArticleType::LeftHand).unwrap()[0];
        save.file.bytes[0x5f8] = 0;
        let result = article.transform(&mut save.file, u32::from_le_bytes([0x40,0x4b,0x4c,0x00]), false);
        assert!(result.is_err());
        if let Err(error) = result {
            assert_eq!(error.to_string(), "Save error: ERROR: The Article was not found above the inventory.");
        }

        //Try to transform a weapon into another that does not exist
        let mut save = build_save_data("testsave0");

        let article = &mut save.inventory.articles.get_mut(&ArticleType::LeftHand).unwrap()[0];
        assert!(check_bytes(&save.file, 0x89ec,
            &[0x4a,0x00,0x83,0x7c,0x51,0x00,0x80,0x80,0x80,0x9f,0xd5,0x00,0x01,0x00,0x00,0x00]));
        assert!(check_bytes(&save.file, 0x5f8,
            &[0x80,0x9f,0xd5,0x00]));
        let result = article.transform(&mut save.file, u32::from_le_bytes([0xAA,0xBB,0xCC,0xDD]), false);
        assert!(result.is_err());
        if let Err(error) = result {
            assert_eq!(error.to_string(), "Save error: ERROR: Failed to find info for the article.");
        }
        article.transform(&mut save.file, u32::from_le_bytes([0x40,0x4b,0x4c,0x00]), false).unwrap();

        assert!(check_bytes(&save.file, 0x89ec,
            &[0x4a,0x00,0x83,0x7c,0x51,0x00,0x80,0x80,0x40,0x4b,0x4c,0x00,0x01,0x00,0x00,0x00]));
        assert!(check_bytes(&save.file, 0x5f8,
            &[0x40,0x4b,0x4c,0x00]));
        assert_eq!(article.id, u32::from_le_bytes([0x40,0x4b,0x4c,0x00]));
        assert_eq!(article.first_part, u32::from_le_bytes([0x51,0x00,0x80,0x80]));
        assert_eq!(article.second_part, u32::from_le_bytes([0x40,0x4b,0x4c,0x00]));

        //transform an armor
        let article = &mut save.inventory.articles.get_mut(&ArticleType::Armor).unwrap()[0];
        assert!(check_bytes(&save.file, 0x898c,
            &[0x44,0xf0,0xff,0xff,0x48,0x00,0x80,0x90,0x70,0x82,0x03,0x10,0x01,0x00,0x00,0x00]));
        assert!(check_bytes(&save.file, 0x3dc,
            &[0x70,0x82,0x03,0x10]));

        article.transform(&mut save.file, u32::from_le_bytes([0x60,0x5b,0x03,0x00]), false).unwrap();

        assert!(check_bytes(&save.file, 0x898c,
            &[0x44,0xf0,0xff,0xff,0x48,0x00,0x80,0x90,0x60,0x5b,0x03,0x10,0x01,0x00,0x00,0x00]));
        assert!(check_bytes(&save.file, 0x3dc,
            &[0x60,0x5b,0x03,0x10]));
        assert_eq!(article.id, u32::from_le_bytes([0x60,0x5b,0x03,0x00]));
        assert_eq!(article.first_part, u32::from_le_bytes([0x48,0x00,0x80,0x90]));
        assert_eq!(article.second_part, u32::from_le_bytes([0x60,0x5b,0x03,0x10]));

        //error tests
        assert!(article.transform_armor_or_weapon(&mut save.file, vec![0xAA,0xBB,0xCC], false).is_err());
        assert!(article.transform_armor_or_weapon(&mut save.file, vec![0xAA,0xBB,0xCC,0xDD,0xEE], false).is_err());

        article.number = 255;
        let result = article.transform_armor_or_weapon(&mut save.file, vec![0xAA,0xBB,0xCC,0xDD], false);
        assert!(result.is_err());
        if let Err(error) = result {
            assert_eq!(error.to_string(), "Save error: ERROR: The Article was not found in the inventory.");
        }
    }

    #[test]
    fn article_is_type_family() {
        let save = build_save_data("testsave0");

        let armor = save.inventory.articles.get(&ArticleType::Armor).unwrap()[0].clone();
        let weapon = save.inventory.articles.get(&ArticleType::LeftHand).unwrap()[0].clone();
        let item = save.inventory.articles.get(&ArticleType::Material).unwrap()[0].clone();
        assert!(armor.is_armor());
        assert!(weapon.is_weapon());
        assert!(item.is_item());
        assert!(!armor.is_weapon());
        assert!(!weapon.is_item());
        assert!(!item.is_armor());
    }

    #[test]
    fn weapon_mods_from() {
        //Uncanny Chikage +1
        let weapon_mods = WeaponMods::try_from(u32::from_le_bytes([0xF4, 0xAB, 0x1E, 0x00])).unwrap();
        assert_eq!(weapon_mods.upgrade_level, 1);
        assert_eq!(weapon_mods.imprint, Some(Imprint::Uncanny));

        //Chikage Saw Cleaver +5
        let weapon_mods = WeaponMods::try_from(u32::from_le_bytes([0xB4, 0xD1, 0x6A, 0x00])).unwrap();
        assert_eq!(weapon_mods.upgrade_level, 5);
        assert_eq!(weapon_mods.imprint, None);

        //Lost Saw Cleaver +0
        let weapon_mods = WeaponMods::try_from(u32::from_le_bytes([0xE0, 0x1D, 0x6B, 0x00])).unwrap();
        assert_eq!(weapon_mods.upgrade_level, 0);
        assert_eq!(weapon_mods.imprint, Some(Imprint::Lost));

        //Lost Holy Moonlight Sword +9
        let weapon_mods = WeaponMods::try_from(u32::from_le_bytes([0x24, 0x0C, 0x8D, 0x01])).unwrap();
        assert_eq!(weapon_mods.upgrade_level, 9);
        assert_eq!(weapon_mods.imprint, Some(Imprint::Lost));
    }

    #[test]
    fn test_scale_weapon_info() {
        let save = build_save_data("weaponmods0");
        let right_hands = save.inventory.articles.get(&ArticleType::RightHand).unwrap();
        assert_eq!(right_hands.len(), 7);

        //Lost Holy Moonlight Sword +9
        let right_hand0 = right_hands[0].info.extra_info.clone().unwrap();
        assert_eq!(right_hand0["damage"]["physical"], Value::from("171"));
        assert_eq!(right_hand0["damage"]["arcane"], Value::from("95"));

        //Chikage Saw Cleaver +5
        let right_hand0 = right_hands[1].info.extra_info.clone().unwrap();
        assert_eq!(right_hand0["damage"]["physical"], Value::from("135"));

        //Lost Saw Cleaver
        let right_hand0 = right_hands[2].info.extra_info.clone().unwrap();
        assert_eq!(right_hand0["damage"]["physical"], Value::from("90"));

        //Chikage Ludwig's Holy Blade +2
        let right_hand0 = right_hands[3].info.extra_info.clone().unwrap();
        assert_eq!(right_hand0["damage"]["physical"], Value::from("120"));

        //Chikage Rifle Spear +6
        let right_hand0 = right_hands[4].info.extra_info.clone().unwrap();
        assert_eq!(right_hand0["damage"]["physical"], Value::from("133"));
        assert_eq!(right_hand0["damage"]["blood"], Value::from("133"));

        //Uncanny Chikage +1
        let right_hand0 = right_hands[5].info.extra_info.clone().unwrap();
        assert_eq!(right_hand0["damage"]["physical"], Value::from("101"));
        assert_eq!(right_hand0["damage"]["blood"], Value::from("101"));

        //Chikage Lugarius' Wheel +7
        let right_hand0 = right_hands[6].info.extra_info.clone().unwrap();
        assert_eq!(right_hand0["damage"]["physical"], Value::from("170"));
        assert_eq!(right_hand0["damage"]["arcane"], Value::from("39"));
    }

    #[test]
    fn article_set_imprint_and_upgrade() {
        //Test with Lost imprint
        let mut save = build_save_data("testsave0");

        let article = &mut save.inventory.articles.get_mut(&ArticleType::LeftHand).unwrap()[0];
        assert!(check_bytes(&save.file, 0x89ec,
            &[0x4a,0x00,0x83,0x7c,0x51,0x00,0x80,0x80,0x80,0x9f,0xd5,0x00,0x01,0x00,0x00,0x00]));
        assert!(check_bytes(&save.file, 0x5f8,
            &[0x80,0x9f,0xd5,0x00]));
        let extra_info = article.info.extra_info.clone().unwrap();
        assert_eq!(extra_info["imprint"], Value::Null);
        assert_eq!(extra_info["upgrade_level"], Value::from(0));
        // 0xD59F80 + 20800 = 0xD5F0C0
        article.set_imprint_and_upgrade(&mut save.file, Some(Some(Imprint::Lost)), Some(8)).unwrap();
        assert!(check_bytes(&save.file, 0x89ec,
            &[0x4a,0x00,0x83,0x7c,0x51,0x00,0x80,0x80,0xC0,0xF0,0xD5,0x00,0x01,0x00,0x00,0x00]));
        assert!(check_bytes(&save.file, 0x5f8,
            &[0xC0,0xF0,0xD5,0x00]));
        assert_eq!(article.id, u32::from_le_bytes([0xC0,0xF0,0xD5,0x00]));
        assert_eq!(article.first_part, u32::from_le_bytes([0x51,0x00,0x80,0x80]));
        assert_eq!(article.second_part, u32::from_le_bytes([0xC0,0xF0,0xD5,0x00]));
        let extra_info = article.info.extra_info.clone().unwrap();
        assert_eq!(extra_info["imprint"], json!(Some(Imprint::Lost)));
        assert_eq!(extra_info["upgrade_level"], Value::from(8));

        //Test with Uncanny imprint
        let mut save = build_save_data("testsave0");

        let article = &mut save.inventory.articles.get_mut(&ArticleType::LeftHand).unwrap()[0];
        // 0xD59F80 + 11000 = 0xD5CA78
        article.set_imprint_and_upgrade(&mut save.file, Some(Some(Imprint::Uncanny)), Some(10)).unwrap();
        assert!(check_bytes(&save.file, 0x89ec,
            &[0x4a,0x00,0x83,0x7c,0x51,0x00,0x80,0x80,0x78,0xCA,0xD5,0x00,0x01,0x00,0x00,0x00]));
        assert!(check_bytes(&save.file, 0x5f8,
            &[0x78,0xCA,0xD5,0x00]));
        assert_eq!(article.id, u32::from_le_bytes([0x78,0xCA,0xD5,0x00]));
        assert_eq!(article.first_part, u32::from_le_bytes([0x51,0x00,0x80,0x80]));
        assert_eq!(article.second_part, u32::from_le_bytes([0x78,0xCA,0xD5,0x00]));
        let extra_info = article.info.extra_info.clone().unwrap();
        assert_eq!(extra_info["imprint"], json!(Some(Imprint::Uncanny)));
        assert_eq!(extra_info["upgrade_level"], Value::from(10));

        //Test errors
        let article = &mut save.inventory.articles.get_mut(&ArticleType::Armor).unwrap()[0];
        let result = article.set_imprint_and_upgrade(&mut save.file, None, None);
        assert!(result.is_err());
        if let Err(error) = result {
            assert_eq!(error.to_string(), "Save error: ERROR: The article must be a weapon.");
        }

        let article = &mut save.inventory.articles.get_mut(&ArticleType::RightHand).unwrap()[0];
        let result = article.set_imprint_and_upgrade(&mut save.file, None, Some(11));
        assert!(result.is_err());
        if let Err(error) = result {
            assert_eq!(error.to_string(), "Save error: ERROR: Upgrade level cannot be bigger than 10.");
        }

        article.info.extra_info = None;
        let result = article.set_imprint_and_upgrade(&mut save.file, None, None);
        assert!(result.is_err());
        if let Err(error) = result {
            assert_eq!(error.to_string(), "Save error: ERROR: The weapon has no extrainfo.");
        }

        article.second_part = u32::MAX;
        let result = article.set_imprint_and_upgrade(&mut save.file, None, None);
        assert!(result.is_err());
        if let Err(error) = result {
            assert_eq!(error.to_string(), "Save error: ERROR: Invalid second_part");
        }

        let article = &mut save.inventory.articles.get_mut(&ArticleType::RightHand).unwrap()[1];
        article.number = 0xEE;
        let result = article.set_imprint_and_upgrade(&mut save.file, None, None);
        assert!(result.is_err());
        if let Err(error) = result {
            assert_eq!(error.to_string(), "Save error: ERROR: The Article was not found in the inventory.");
        }

        let article = &mut save.inventory.articles.get_mut(&ArticleType::LeftHand).unwrap()[0];
        save.file.bytes[0x5f8] = 0x00;
        let result = article.set_imprint_and_upgrade(&mut save.file, None, None);
        assert!(result.is_err());
        if let Err(error) = result {
            assert_eq!(error.to_string(), "Save error: ERROR: The weapon was not found above the inventory.");
        }

    }

    #[test]
    fn article_change_slot_shape() {
        let mut save = build_save_data("testsave9");
        let mut consumable = save.inventory.articles.get(&ArticleType::Consumable).unwrap()[0].clone();
        let hunter_axe = save.inventory.articles.get_mut(&ArticleType::RightHand).unwrap().get_mut(0).unwrap();
        let hunter_axe_reference = hunter_axe.clone();

        //Test error cases
        let result = hunter_axe.change_slot_shape(&mut save.file, 500, SlotShape::Closed);
        assert!(result.is_err());
        if let Err(error) = result {
            assert_eq!(error.to_string(), "Save error: ERROR: Invalid slot_index.");
        }
        assert_eq!(*hunter_axe, hunter_axe_reference);

        //Test error cases
        let mut file_data = build_file_data("testsave0");
        let result = hunter_axe.change_slot_shape(&mut file_data, 0, SlotShape::Closed);
        assert!(result.is_err());
        if let Err(error) = result {
            assert_eq!(error.to_string(), "Save error: ERROR: Failed to find the article in the file data.");
        }
        assert_eq!(*hunter_axe, hunter_axe_reference);

        //Test error cases
        let result = consumable.change_slot_shape(&mut save.file, 0, SlotShape::Closed);
        assert!(result.is_err());
        if let Err(error) = result {
            assert_eq!(error.to_string(), "Save error: ERROR: Article does not have slots.");
        }

        assert_eq!(hunter_axe.slots.as_ref().unwrap()[0].shape, SlotShape::Radial);
        assert!(check_bytes(&save.file, 0x1570,
            &[0xd0, 0x01, 0x80, 0x80,
              0x6c, 0x4c, 0x4c, 0x00,
              0xfa, 0x00, 0x00, 0x00,
              0x00, 0x00, 0x00, 0x00,
              0x01, 0x00, 0x00, 0x00,
              0x01, 0x00, 0x00, 0x00,
              0x74, 0x00, 0x80, 0xc0,
              0x01, 0x00, 0x00, 0x00,
              0x6f, 0x00, 0x80, 0xc0,
              0x00, 0x00, 0x00, 0x80,
              0x00, 0x00, 0x00, 0x00,
              0x00, 0x00, 0x00, 0x80,
              0x00, 0x00, 0x00, 0x00,
              0x00, 0x00, 0x00, 0x80,
              0x00, 0x00, 0x00, 0x00,
              0xd1, 0x01, 0x80, 0x80,
              0x00, 0x12, 0x7a, 0x00,
              0xfa, 0x00, 0x00, 0x00,
              0x00, 0x00, 0x00, 0x00,
              0x01, 0x00, 0x00, 0x00,
              0x00, 0x00, 0x00, 0x80,
              0x00, 0x00, 0x00, 0x00,
              0x00, 0x00, 0x00, 0x80,
              0x00, 0x00, 0x00, 0x00,
              0x00, 0x00, 0x00, 0x80,
              0x00, 0x00, 0x00, 0x00,
              0x00, 0x00, 0x00, 0x80,
              0x00, 0x00, 0x00, 0x00,
              0x00, 0x00, 0x00, 0x80,
              0x00, 0x00, 0x00, 0x00,
              0xd2, 0x01, 0x80, 0x90,
              0x40, 0x19, 0x01, 0x10,
        ]));

        hunter_axe.change_slot_shape(&mut save.file, 0, SlotShape::Droplet).unwrap();

        assert_eq!(hunter_axe.slots.as_ref().unwrap()[0].shape, SlotShape::Droplet);
        assert!(check_bytes(&save.file, 0x1570,
            &[0xd0, 0x01, 0x80, 0x80,
              0x6c, 0x4c, 0x4c, 0x00,
              0xfa, 0x00, 0x00, 0x00,
              0x00, 0x00, 0x00, 0x00,
              0x01, 0x00, 0x00, 0x00,
              0x3f, 0x00, 0x00, 0x00,
              0x74, 0x00, 0x80, 0xc0,
              0x01, 0x00, 0x00, 0x00,
              0x6f, 0x00, 0x80, 0xc0,
              0x00, 0x00, 0x00, 0x80,
              0x00, 0x00, 0x00, 0x00,
              0x00, 0x00, 0x00, 0x80,
              0x00, 0x00, 0x00, 0x00,
              0x00, 0x00, 0x00, 0x80,
              0x00, 0x00, 0x00, 0x00,
              0xd1, 0x01, 0x80, 0x80,
              0x00, 0x12, 0x7a, 0x00,
              0xfa, 0x00, 0x00, 0x00,
              0x00, 0x00, 0x00, 0x00,
              0x01, 0x00, 0x00, 0x00,
              0x00, 0x00, 0x00, 0x80,
              0x00, 0x00, 0x00, 0x00,
              0x00, 0x00, 0x00, 0x80,
              0x00, 0x00, 0x00, 0x00,
              0x00, 0x00, 0x00, 0x80,
              0x00, 0x00, 0x00, 0x00,
              0x00, 0x00, 0x00, 0x80,
              0x00, 0x00, 0x00, 0x00,
              0x00, 0x00, 0x00, 0x80,
              0x00, 0x00, 0x00, 0x00,
              0xd2, 0x01, 0x80, 0x90,
              0x40, 0x19, 0x01, 0x10,
        ]));
    }
}
