use serde::{Deserialize, Serialize};
use serde_json::{self, json, Value};
use super::{enums::{ArticleType, Error}, file::FileData};
use std::{fs::File,
          io::BufReader,
          collections::HashMap};
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
    pub info: ItemInfo,
    pub article_type: ArticleType,
}

impl Article {
    pub fn transform(&mut self, file_data: &mut FileData, new_id: u32) -> Result<(), Error>{
        let mut new_id = new_id.to_le_bytes().to_vec();
        match self.article_type {
            ArticleType::Consumable | ArticleType::Material | ArticleType::Chalice => {
                new_id.pop();
                self.transform_item(file_data, new_id)
            },
            ArticleType::Armor | ArticleType::RightHand | ArticleType::LeftHand => self.transform_armor_or_weapon(file_data, new_id),
            _ => Err(Error::CustomError("ERROR: This type of article cannot be transformed.")),
        }
    }
    fn transform_item(&mut self, file_data: &mut FileData, new_id: Vec<u8>) -> Result<(), Error>{
        if new_id.len()!=3 {
            Err(Error::CustomError("ERROR: 'new_id' argument must be 3B long."))
        } else {
            let (start, finish) = file_data.offsets.inventory;
            for i in (start..finish).step_by(16) {
                if self.index == file_data.bytes[i] {

                    //ID
                    let id = u32::from_le_bytes([new_id[0],new_id[1],new_id[2],0]);

                    let info;
                    let article_type;
                    //INFO & ARTICLE_TYPE
                    if let Ok((new_info, new_article_type)) = get_info_item(id) {
                        info = new_info;
                        article_type = new_article_type;
                    } else {
                        return Err(Error::CustomError("ERROR: Failed to find info for the item."));
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
                    self.id = id;
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
            let (start, finish) = file_data.offsets.inventory;
            for i in (start..finish).step_by(16) {
                if self.index == file_data.bytes[i] {

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

                    if self.article_type == ArticleType::Armor {
                        second_part = u32::from_le_bytes([new_id[0],new_id[1],new_id[2],0x10]);
                        byte_count = 3;
                        result = get_info_armor(id);
                    } else {
                        second_part = u32::from_le_bytes([new_id[0],new_id[1],new_id[2],new_id[3]]);
                        result = get_info_weapon(id);
                    }

                    //INFO & ARTICLE_TYPE
                    if let Ok((new_info, new_article_type)) = result {
                        info = new_info;
                        article_type = new_article_type;
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
                    self.id = id;
                    self.info = info;
                    self.second_part = second_part;
                    self.article_type = article_type;
                    return Ok(())
                }
            }
            Err(Error::CustomError("ERROR: The Article was not found in the inventory."))
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Inventory {
    pub articles: HashMap<ArticleType, Vec<Article>>,
}

impl Inventory {
    ///Modifies the amount of an article
    pub fn edit_item(&mut self, file_data: &mut FileData, index: u8, value: u32) -> Result<(), Error> {
        let value_endian = u32::to_le_bytes(value);
        let (start, _) = file_data.offsets.inventory;
        let mut found = false;
        for (k, v) in self.articles.iter_mut() {
            if (k == &ArticleType::Consumable) || (k == &ArticleType::Material) || (k == &ArticleType::Chalice) || (k == &ArticleType::Key) {
                if let Some(item) = v.iter_mut().find(|item| item.index == index) {
                    if k == &ArticleType::Key {
                        return Err(Error::CustomError("ERROR: Key items cannot be edited."));
                    }
                    item.amount = value;
                    found = true;
                    break;
                }
            }
        }
        if !found {
            return Err(Error::CustomError("ERROR: The Article was not found in the inventory."));
        }
        for i in (start..file_data.bytes.len()).step_by(16) {
            if index == file_data.bytes[i] {
                for (i, b) in file_data.bytes[i + 12..i + 16].iter_mut().enumerate() {
                    *b = value_endian[i];
                }
                break;
            }
        }
        Ok(())
    }

    pub fn _add_item(&mut self, file_data: &mut FileData, id: u32, quantity: u32) -> Result<(), Error> {
        let (_, inventory_end) = file_data.offsets.inventory;
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

        let id = u32::from_le_bytes(endian_id);

        if let Ok((info, article_type)) = get_info_item(id) {
            let new_item = Article {
                index: file_data.bytes[inventory_end + 12],
                id,
                first_part: 213,
                second_part: 1233,
                info,
                amount: quantity,
                article_type,
            };
            self.articles.entry(article_type).or_insert(Vec::new()).push(new_item);
            return Ok(());
        }
        Err(Error::CustomError("ERROR: failed to find info for the item."))
    }
}

pub fn build(file_data: &FileData) -> Inventory {
    let mut articles = parse_articles(file_data);
    let mut key_items = parse_key_inventory(file_data);
    if !key_items.is_empty() {
        articles.entry(ArticleType::Key).or_insert(Vec::new()).append(&mut key_items);
    }
    Inventory {
        articles,
    }
}

pub fn parse_articles(file_data: &FileData) -> HashMap<ArticleType, Vec<Article>> {
    let mut articles = HashMap::new();
    let (inventory_start, _) = file_data.offsets.inventory;
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

        let result = match (file_data.bytes[i+7],file_data.bytes[i+11]) {
            (0xB0,0x40) => get_info_item(id),
            (_,0x10) => get_info_armor(id),
            _ => {
                id = second_part;
                get_info_weapon(id)
            },
        };

        if let Ok((info, article_type)) = result {
            let article = Article {
                index,
                id,
                first_part,
                second_part,
                amount,
                info,
                article_type,
            };
            let category = articles.entry(article_type).or_insert(Vec::new());
            category.push(article);
        };
    }
    articles
}

pub fn parse_key_inventory(file_data: &FileData) -> Vec<Article> {
    let mut articles = Vec::new();
    let (inventory_start, _) = file_data.offsets.key_inventory;
    for i in (inventory_start..file_data.bytes.len()).step_by(16) {
        let index = file_data.bytes[i] + 1;
        let id = u32::from_le_bytes([file_data.bytes[i + 8], file_data.bytes[i + 9], file_data.bytes[i + 10], 0]);
        let first_part =
            u32::from_le_bytes([file_data.bytes[i + 4], file_data.bytes[i + 5], file_data.bytes[i + 6], file_data.bytes[i + 7]]);
        let second_part =
            u32::from_le_bytes([file_data.bytes[i + 8], file_data.bytes[i + 9], file_data.bytes[i + 10], file_data.bytes[i + 11]]);
        let amount =
            u32::from_le_bytes([file_data.bytes[i + 12], file_data.bytes[i + 13], file_data.bytes[i + 14], file_data.bytes[i + 15]]);

        if first_part == 0 && second_part == u32::MAX && amount == 0 {
            break; //The last article is discarded
        }
        if let Ok((info, article_type)) = get_info_item(id) {
            articles.push(Article {
                index,
                id,
                first_part,
                second_part,
                amount,
                info,
                article_type,
            });
        };
    }
    //The index of the first article needs to be set manually
    if let Some(art) = articles.get_mut(0) {
        art.index = 0;
    }
    articles
}

pub fn get_info_item(id: u32) -> Result<(ItemInfo, ArticleType), Error> {
    let json_file = File::open("items.json").map_err(Error::IoError)?;
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
                return Ok((info, ArticleType::from_string(&category)))
            },
            None => ()
        }
    }
    Err(Error::CustomError("ERROR: Failed to find info for the item."))
}

pub fn get_info_armor(id: u32) -> Result<(ItemInfo, ArticleType), Error> {
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
            return Ok((info, ArticleType::Armor))
        },
        None => ()
    }
    Err(Error::CustomError("ERROR: Failed to find info for the armor."))
}

pub fn get_info_weapon(id: u32) -> Result<(ItemInfo, ArticleType), Error> {
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
                return Ok((info, ArticleType::from_string(&category)))
            },
            None => ()
        }
    }
    Err(Error::CustomError("ERROR: Failed to find info for the weapon."))
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

        let article = &mut inventory.articles.get_mut(&ArticleType::Consumable).unwrap()[0];
        assert!(check_bytes(&file_data, 0x89cc,
            &[0x48,0x80,0xCF,0xA8,0x64,0,0,0xB0,0x64,0,0,0x40,0x01,0,0,0]));
        let result = article.transform_item(&mut file_data, vec![0xAA,0xBB,0xCC]);
        assert!(result.is_err());
        if let Err(error) = result {
            assert_eq!(error.to_string(), "Save error: ERROR: Failed to find info for the item.");
        }
        article.transform_item(&mut file_data, vec![0x64,0x1B,0x00]).unwrap();
        assert!(check_bytes(&file_data, 0x89cc,
            &[0x48,0x80,0xCF,0xA8,0x64,0x1B,0x00,0xB0,0x64,0x1B,0x00,0x40,0x01,0,0,0]));
        assert_eq!(article.id, u32::from_le_bytes([0x64,0x1B,0x00,0x00]));
        assert_eq!(article.first_part, u32::from_le_bytes([0x64,0x1B,0x00,0xB0]));
        assert_eq!(article.second_part, u32::from_le_bytes([0x64,0x1B,0x00,0x40]));
        assert_eq!(article.article_type, ArticleType::Material);

        //error tests
        assert!(article.transform_item(&mut file_data, vec![0xAA,0xBB]).is_err());
        assert!(article.transform_item(&mut file_data, vec![0xAA,0xBB,0xCC,0xDD]).is_err());
        article.index = 255;
        assert!(article.transform_item(&mut file_data, vec![0xAA,0xBB,0xCC]).is_err());

        //test transforming a Consumable into a Key
        let article = &mut inventory.articles.get_mut(&ArticleType::Consumable).unwrap()[1];
        assert_eq!(article.amount, 20);
        assert!(check_bytes(&file_data, 0x89FC,
            &[0x4B,0x00,0xFD,0x7F,0x84,0x03,0x00,0xB0,0x84,0x03,0x00,0x40,0x14,0x00,0x00,0x00]));
        article.transform_item(&mut file_data, vec![0xA0,0x0F,0x00]).unwrap();
        assert!(check_bytes(&file_data, 0x89FC,
            &[0x4B,0x00,0xFD,0x7F,0xA0,0x0F,0x00,0xB0,0xA0,0x0F,0x00,0x40,0x01,0x00,0x00,0x00]));
        assert_eq!(article.article_type, ArticleType::Key);
        assert_eq!(article.amount, 1);
    }

    #[test]
    fn article_transform_armor_or_weapon() {
        let mut file_data = build_file_data();
        let mut inventory = build(&file_data);

        let article = &mut inventory.articles.get_mut(&ArticleType::LeftHand).unwrap()[0];
        assert!(check_bytes(&file_data, 0x89ec, 
            &[0x4a,0x00,0x83,0x7c,0x51,0x00,0x80,0x80,0x80,0x9f,0xd5,0x00,0x01,0x00,0x00,0x00]));
        assert!(check_bytes(&file_data, 0x5f8, 
            &[0x80,0x9f,0xd5,0x00]));
        let result = article.transform_armor_or_weapon(&mut file_data, vec![0xAA,0xBB,0xCC,0xDD]);
        assert!(result.is_err());
        if let Err(error) = result {
            assert_eq!(error.to_string(), "Save error: ERROR: Failed to find info for the article.");
        }
        article.transform_armor_or_weapon(&mut file_data, vec![0x40,0x4b,0x4c,0x00]).unwrap();

        assert!(check_bytes(&file_data, 0x89ec, 
            &[0x4a,0x00,0x83,0x7c,0x51,0x00,0x80,0x80,0x40,0x4b,0x4c,0x00,0x01,0x00,0x00,0x00]));
        assert!(check_bytes(&file_data, 0x5f8, 
            &[0x40,0x4b,0x4c,0x00]));
        assert_eq!(article.id, u32::from_le_bytes([0x40,0x4b,0x4c,0x00]));
        assert_eq!(article.first_part, u32::from_le_bytes([0x51,0x00,0x80,0x80]));
        assert_eq!(article.second_part, u32::from_le_bytes([0x40,0x4b,0x4c,0x00]));

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
        assert!(check_bytes(&file_data, 0x89cc, 
            &[0x48,0x80,0xCF,0xA8,0x64,0,0,0xB0,0x64,0,0,0x40,0x01,0,0,0]));
        //Try to edit a key item
        let result = inventory.edit_item(&mut file_data, 0x00, 0xAABBCCDD);
        assert!(result.is_err());
        if let Err(error) = result {
            assert_eq!(error.to_string(), "Save error: ERROR: Key items cannot be edited.");
        }
        //Try wrong index
        let result = inventory.edit_item(&mut file_data, 0xFF, 0xAABBCCDD);
        assert!(result.is_err());
        if let Err(error) = result {
            assert_eq!(error.to_string(), "Save error: ERROR: The Article was not found in the inventory.");
        }

        assert!(inventory.edit_item(&mut file_data, 0x48, 0xAABBCCDD).is_ok());
        assert!(check_bytes(&file_data, 0x89cc, 
            &[0x48,0x80,0xCF,0xA8,0x64,0,0,0xB0,0x64,0,0,0x40,0xDD,0xCC,0xBB,0xAA]));
        assert_eq!(inventory.articles.get(&ArticleType::Consumable).unwrap()[0].amount, 0xAABBCCDD);
    }

    #[test]
    fn test_parse_key_inventory() {
        let file_data = FileData::build("saves/testsave0").unwrap();
        let articles = parse_key_inventory(&file_data);
        assert_eq!(articles.len(), 6);

        //Item N0
        assert_eq!(articles[0].index, 0);
        assert_eq!(articles[0].id, u32::from_le_bytes([0x12, 0x10, 0x00, 0x00]));
        assert_eq!(articles[0].first_part, u32::from_le_bytes([0x12, 0x10, 0x00, 0xb0]));
        assert_eq!(articles[0].second_part, u32::from_le_bytes([0x12, 0x10, 0x00, 0x40]));
        assert_eq!(articles[0].amount, u32::from_le_bytes([0x01, 0x00, 0x00, 0x00]));
        assert_eq!(articles[0].article_type, ArticleType::Key);

        //Item N1
        assert_eq!(articles[1].index, 1);
        assert_eq!(articles[1].id, u32::from_le_bytes([0xd8, 0x10, 0x00, 0x00]));
        assert_eq!(articles[1].first_part, u32::from_le_bytes([0xd8, 0x10, 0x00, 0xb0]));
        assert_eq!(articles[1].second_part, u32::from_le_bytes([0xd8, 0x10, 0x00, 0x40]));
        assert_eq!(articles[1].amount, u32::from_le_bytes([0x01, 0x00, 0x00, 0x00]));
        assert_eq!(articles[1].article_type, ArticleType::Key);

        //Item N2
        assert_eq!(articles[2].index, 2);
        assert_eq!(articles[2].id, u32::from_le_bytes([0x0e, 0x10, 0x00, 0x00]));
        assert_eq!(articles[2].first_part, u32::from_le_bytes([0x0e, 0x10, 0x00, 0xb0]));
        assert_eq!(articles[2].second_part, u32::from_le_bytes([0x0e, 0x10, 0x00, 0x40]));
        assert_eq!(articles[2].amount, u32::from_le_bytes([0x01, 0x00, 0x00, 0x00]));
        assert_eq!(articles[2].article_type, ArticleType::Key);

        //Item N3
        assert_eq!(articles[3].index, 3);
        assert_eq!(articles[3].id, u32::from_le_bytes([0xa0, 0x0f, 0x00, 0x00]));
        assert_eq!(articles[3].first_part, u32::from_le_bytes([0xa0, 0x0f, 0x00, 0xb0]));
        assert_eq!(articles[3].second_part, u32::from_le_bytes([0xa0, 0x0f, 0x00, 0x40]));
        assert_eq!(articles[3].amount, u32::from_le_bytes([0x01, 0x00, 0x00, 0x00]));
        assert_eq!(articles[3].article_type, ArticleType::Key);

        //Item N4
        assert_eq!(articles[4].index, 4);
        assert_eq!(articles[4].id, u32::from_le_bytes([0x07, 0x10, 0x00, 0x00]));
        assert_eq!(articles[4].first_part, u32::from_le_bytes([0x07, 0x10, 0x00, 0xb0]));
        assert_eq!(articles[4].second_part, u32::from_le_bytes([0x07, 0x10, 0x00, 0x40]));
        assert_eq!(articles[4].amount, u32::from_le_bytes([0x01, 0x00, 0x00, 0x00]));
        assert_eq!(articles[4].article_type, ArticleType::Key);

        //Item N5
        assert_eq!(articles[5].index, 5);
        assert_eq!(articles[5].id, u32::from_le_bytes([0xab, 0x0f, 0x00, 0x00]));
        assert_eq!(articles[5].first_part, u32::from_le_bytes([0xab, 0x0f, 0x00, 0xb0]));
        assert_eq!(articles[5].second_part, u32::from_le_bytes([0xab, 0x0f, 0x00, 0x40]));
        assert_eq!(articles[5].amount, u32::from_le_bytes([0x01, 0x00, 0x00, 0x00]));
        assert_eq!(articles[5].article_type, ArticleType::Key);
    }
}
