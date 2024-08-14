use serde::{Deserialize, Serialize};
use serde_json::{self, json, Value};
use super::{enums::{ArticleType, Error}, file::FileData, constants::USERNAME_TO_KEY_INV_OFFSET};
use std::{fs::File, io::BufReader};
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ItemInfo {
    pub item_name: String,
    pub item_desc: String,
    pub item_img: String,
    pub extra_info: Option<Value>
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Article {
    // pub name: String,
    pub index: u8,
    pub id: u32,
    pub first_part: u32,
    pub second_part: u32,
    pub amount: u32,
    pub info: Option<ItemInfo>,
    pub article_type: ArticleType,
}

impl Article {
    pub fn transform(&mut self, file_data: &mut FileData, new_id: u32) -> Result<(), Error>{
        let mut new_id = new_id.to_le_bytes().to_vec();
        match self.article_type {
            ArticleType::Item => {
                new_id.pop();
                self.transform_item(file_data, new_id)
            },
            ArticleType::Armor | ArticleType::Weapon => self.transform_armor_or_weapon(file_data, new_id),
        }
    }
    fn transform_item(&mut self, file_data: &mut FileData, new_id: Vec<u8>) -> Result<(), Error>{
        if new_id.len()!=3 {
            Err(Error::CustomError("ERROR: 'new_id' argument must be 3B long."))
        } else {
            let (start, finish) = inventory_offset(&file_data);
            for i in (start..finish).step_by(16) {
                if self.index == file_data.bytes[i] {

                    //FIRST PART
                    for j in i+4..=i+6 {
                        file_data.bytes[j] = new_id[j-i-4];
                    }
                    self.first_part = u32::from_le_bytes([new_id[0],new_id[1],new_id[2],0xB0]);
                    
                    //SECOND PART
                    for j in i+8..=i+10 {
                        file_data.bytes[j] = new_id[j-i-8];
                    }
                    self.second_part = u32::from_le_bytes([new_id[0],new_id[1],new_id[2],0x40]);
                    
                    //ID
                    self.id = u32::from_le_bytes([new_id[0],new_id[1],new_id[2],0]);

                    //INFO
                    self.info = get_info(self.id, &self.article_type)?;
                    return Ok(())
                }
            }
            Err(Error::CustomError("ERROR: The Article was not found in the inventory."))
        }
    }

    fn transform_armor_or_weapon(&mut self, file_data: &mut FileData, new_id: Vec<u8>) -> Result<(), Error>{
        if new_id.len()!=4 {
            Err(Error::CustomError("ERROR: 'new_id' argument must be 4B long."))
        } else {
            let (start, finish) = inventory_offset(&file_data);
            for i in (start..finish).step_by(16) {
                if self.index == file_data.bytes[i] {

                    //Take the first and second part to search later
                    let mut query = Vec::with_capacity(8);
                    let mut byte_count = 4;
                    
                    for j in 4..=11 {
                        query.push(file_data.bytes[i+j]);
                    }
                    
                    if self.article_type == ArticleType::Armor {
                        self.second_part = u32::from_le_bytes([new_id[0],new_id[1],new_id[2],0x10]);
                        byte_count = 3;
                    } else {
                        self.second_part = u32::from_le_bytes([new_id[0],new_id[1],new_id[2],new_id[3]]);
                    }
                    //SECOND PART
                    for j in i+8..i+8+byte_count {
                        file_data.bytes[j] = new_id[j-i-8];
                    }
                    
                    //ID
                    self.id = u32::from_le_bytes([new_id[0],new_id[1],new_id[2],new_id[3]]);

                    //INFO
                    self.info = get_info(self.id, &self.article_type)?;

                    //Search for the query above the inventory (where the article appears with its gems)
                    let mut found = false;
                    let mut index = 0;
                    for j in (0..(start - 8)).rev() {
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
                    return Ok(())
                }
            }
            Err(Error::CustomError("ERROR: The Article was not found in the inventory."))
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Inventory {
    pub articles: Vec<Article>,
}

impl Inventory {
    ///Modifies the amount of an article
    pub fn edit_item(&mut self, file_data: &mut FileData, index: u8, value: u32) {
        let value_endian = u32::to_le_bytes(value);
        let (start, _) = inventory_offset(&file_data);
        for i in (start..file_data.bytes.len()).step_by(16) {
            if index == file_data.bytes[i] {
                for (i, b) in file_data.bytes[i + 12..i + 16].iter_mut().enumerate() {
                    if let Some(item) = self.articles.iter_mut().find(|item| item.index == index) {
                        item.amount = value;
                    }
                    *b = value_endian[i];
                }
                break;
            }
        }
    }

    pub fn _add_item(&mut self, file_data: &mut FileData, id: u32, quantity: u32) {
        let (_, inventory_end) = inventory_offset(file_data);
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
        file_data.bytes[inventory_end + 12] = file_data.bytes[inventory_end - 4] + 1;

        let new_item = Article {
            index: file_data.bytes[inventory_end + 12],
            id: u32::from_le_bytes(endian_id),
            first_part: 213,
            second_part: 1233,
            info: get_info(id, &ArticleType::Item).unwrap(),
            amount: quantity,
            article_type: ArticleType::Item,
        };

        self.articles.push(new_item);
    }
}

pub fn build(file_data: &FileData) -> Inventory {
    Inventory {
        articles: parse_articles(file_data),
    }
}

pub fn inventory_offset(file_data: &FileData) -> (usize, usize) {
    let mut matches: (usize, usize) = (0, 0);
    for i in file_data.username_offset..file_data.bytes.len() - 4 {
        let mut buffer = [0; 4];
        buffer[..4].copy_from_slice(&file_data.bytes[i..i + 4]);
        let current = u32::from_le_bytes(buffer);
        let t = 0xfffff040 as u32;
        let e = 0xffffffff as u32;
        if t == current {
            if matches.0 == 0 {
                matches.0 = i;
            }
        } else if e == current && matches.0 != 0 {
            matches.1 = i + 7;
            break;
        }
    }
    matches
}

pub fn key_inventory_offset(file_data: &FileData) -> usize {
    file_data.username_offset + USERNAME_TO_KEY_INV_OFFSET
}

pub fn parse_articles(file_data: &FileData) -> Vec<Article> {
    let mut articles = Vec::new();
    let (inventory_start, _) = inventory_offset(file_data);
    for i in (inventory_start..file_data.bytes.len()).step_by(16) {
        let index = file_data.bytes[i];
        let mut id = u32::from_le_bytes([file_data.bytes[i + 8], file_data.bytes[i + 9], file_data.bytes[i + 10], 0]);
        let first_part =
            u32::from_le_bytes([file_data.bytes[i + 4], file_data.bytes[i + 5], file_data.bytes[i + 6], file_data.bytes[i + 7]]);
        let second_part =
            u32::from_le_bytes([file_data.bytes[i + 8], file_data.bytes[i + 9], file_data.bytes[i + 10], file_data.bytes[i + 11]]);
        let amount =
            u32::from_le_bytes([file_data.bytes[i + 12], file_data.bytes[i + 13], file_data.bytes[i + 14], file_data.bytes[i + 15]]);

        if first_part == 0 && second_part == u32::MAX && amount == 0 {
            break;
        }

        let article_type = match (file_data.bytes[i+7],file_data.bytes[i+11]) {
            (0xB0,0x40) => ArticleType::Item,
            (_,0x10) => ArticleType::Armor,
            _ => {
                id = second_part;
                ArticleType::Weapon
            },
        };

        let info = get_info(id, &article_type).unwrap();
        
        articles.push(Article {
            index,
            id,
            first_part,
            second_part,
            amount,
            info,
            article_type,
        });
    }

    articles
}

pub fn get_info(id: u32, tipo: &ArticleType) -> Result<Option<ItemInfo>, Error> {

    match tipo {
        ArticleType::Item => {
            let json_file = File::open("items.json").map_err(Error::IoError)?;
            let reader = BufReader::new(json_file);
            let items: Value = serde_json::from_reader(reader).unwrap();
            let items = items.as_object().unwrap();
            
              // Can add the category to the info later
            for (category, category_items) in items {
                match category_items.as_object().unwrap().keys().find(|x| x.parse::<u32>().unwrap() == id) {
                    Some(found) => return Ok(Some(serde_json::from_value(category_items[found].clone()).unwrap())),
                    None => ()
                }   
            }
        },
        ArticleType::Weapon => {
            let json_file = File::open("weapons.json").map_err(Error::IoError)?;
            let reader = BufReader::new(json_file);
            let weapons: Value = serde_json::from_reader(reader).unwrap();
            let weapons = weapons.as_object().unwrap();
        
            for (category, category_weapons) in weapons {
                match category_weapons.as_object().unwrap().keys().find(|x| x.parse::<u32>().unwrap() == id) {
                    Some(found) => {
                        let mut info: ItemInfo = serde_json::from_value(category_weapons[found].clone()).unwrap();
                        info.extra_info = Some(json!({
                            "damage": &category_weapons[found]["damage"]
                        }));
                        return Ok(Some(info))
                    },
                    None => ()
                }   
            }
        },
        ArticleType::Armor => {
            let json_file = File::open("armors.json").map_err(Error::IoError)?;
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
                    return Ok(Some(info))
                },
                None => ()
            }  
        }
    }

    Ok(None)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_SAVE_PATH: &str = "saves/testsave0";

    fn build_file_data() -> FileData {
        FileData::build(TEST_SAVE_PATH).unwrap()
    }

    fn check_bytes(file_data: &FileData,index: usize,bytes: &[u8]) -> bool {
        let mut equal = true;
        for (i, byte) in bytes.iter().enumerate() {
            if file_data.bytes[index+i]!=*byte {
                equal = false;
                break;
            }
        }
        equal
    }

    #[test]
    fn article_transform_item() {
        let mut file_data = build_file_data();
        let mut inventory = build(&file_data);

        let article = &mut inventory.articles[8];
        assert!(check_bytes(&file_data, 35276, 
            &[0x48,0x80,0xCF,0xA8,0x64,0,0,0xB0,0x64,0,0,0x40,0x01,0,0,0]));
        article.transform_item(&mut file_data, vec![0xAA,0xBB,0xCC]).unwrap();
        assert!(check_bytes(&file_data, 35276, 
            &[0x48,0x80,0xCF,0xA8,0xAA,0xBB,0xCC,0xB0,0xAA,0xBB,0xCC,0x40,0x01,0,0,0]));
        assert_eq!(article.id, u32::from_le_bytes([0xAA,0xBB,0xCC,0x00]));
        assert_eq!(article.first_part, u32::from_le_bytes([0xAA,0xBB,0xCC,0xB0]));
        assert_eq!(article.second_part, u32::from_le_bytes([0xAA,0xBB,0xCC,0x40]));

        //error tests
        assert!(article.transform_item(&mut file_data, vec![0xAA,0xBB]).is_err());
        assert!(article.transform_item(&mut file_data, vec![0xAA,0xBB,0xCC,0xDD]).is_err());
        
        article.index = 255;
        assert!(article.transform_item(&mut file_data, vec![0xAA,0xBB,0xCC]).is_err());
    }

    #[test]
    fn article_transform_armor_or_weapon() {
        let mut file_data = build_file_data();
        let mut inventory = build(&file_data);

        let article = &mut inventory.articles[0];
        assert!(check_bytes(&file_data, 35148, 
            &[0x40,0xF0,0xFF,0xFF,0x4B,0,0x80,0x80,0x40,0x42,0x0F,0,1,0,0,0]));
        assert!(check_bytes(&file_data, 1164, 
            &[0x4B,0,0x80,0x80,0x40,0x42,0x0F,0]));
        article.transform_armor_or_weapon(&mut file_data, vec![0xAA,0xBB,0xCC,0xDD]).unwrap();
        assert!(check_bytes(&file_data, 35148, 
            &[0x40,0xF0,0xFF,0xFF,0x4B,0,0x80,0x80,0xAA,0xBB,0xCC,0xDD,1,0,0,0]));
        assert!(check_bytes(&file_data, 1164, 
            &[0x4B,0,0x80,0x80,0xAA,0xBB,0xCC,0xDD]));
        assert_eq!(article.id, u32::from_le_bytes([0xAA,0xBB,0xCC,0xDD]));
        assert_eq!(article.first_part, u32::from_le_bytes([0x4B,0,0x80,0x80]));
        assert_eq!(article.second_part, u32::from_le_bytes([0xAA,0xBB,0xCC,0xDD]));

        //error tests
        assert!(article.transform_armor_or_weapon(&mut file_data, vec![0xAA,0xBB,0xCC]).is_err());
        assert!(article.transform_armor_or_weapon(&mut file_data, vec![0xAA,0xBB,0xCC,0xDD,0xEE]).is_err());
        
        article.index = 255;
        assert!(article.transform_armor_or_weapon(&mut file_data, vec![0xAA,0xBB,0xCC]).is_err());
    }
    
    #[test]
    fn inventory_edit_item() {
        let mut file_data = build_file_data();
        let mut inventory = build(&file_data);
        assert!(check_bytes(&file_data, 35276, 
            &[0x48,0x80,0xCF,0xA8,0x64,0,0,0xB0,0x64,0,0,0x40,0x01,0,0,0]));
        inventory.edit_item(&mut file_data, 0x48, 0xAABBCCDD);
        assert!(check_bytes(&file_data, 35276, 
            &[0x48,0x80,0xCF,0xA8,0x64,0,0,0xB0,0x64,0,0,0x40,0xDD,0xCC,0xBB,0xAA]));
        assert_eq!(inventory.articles[8].amount, 0xAABBCCDD);
    }

    #[test]
    fn test_key_inventory_offset() {
        //testsave0
        let file_data = FileData::build("saves/testsave0").unwrap();
        assert_eq!(key_inventory_offset(&file_data), 66880);

        //testsave1
        let file_data = FileData::build("saves/testsave1").unwrap();
        assert_eq!(key_inventory_offset(&file_data), 75252);

        //testsave2
        let file_data = FileData::build("saves/testsave2").unwrap();
        assert_eq!(key_inventory_offset(&file_data), 75320);

        //testsave3
        let file_data = FileData::build("saves/testsave3").unwrap();
        assert_eq!(key_inventory_offset(&file_data), 78396);

        //testsave4
        let file_data = FileData::build("saves/testsave4").unwrap();
        assert_eq!(key_inventory_offset(&file_data), 83496);
    }
}
