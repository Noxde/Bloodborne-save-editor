use serde::Deserialize;
use serde_json::{self, Value};
use super::{enums::{ArticleType, Error}, file::FileData};
use std::{fs::File, io::BufReader};

#[derive(Deserialize, Debug)]
pub struct ItemInfo {
    pub item_name: String,
    pub item_desc: String,
    pub item_img: String,
}

#[derive(Debug)]
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

#[derive(Debug)]
pub struct Inventory {
    pub articles: Vec<Article>,
}

impl Inventory {
    pub fn _edit_item(&mut self, file_data: &mut FileData, index: u8, value: u32) {
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
            info: get_info(id).unwrap(),
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
            matches.1 = i - 4;
            break;
        }
    }
    matches
}

pub fn parse_articles(file_data: &FileData) -> Vec<Article> {
    let mut articles = Vec::new();
    let (inventory_start, _) = inventory_offset(file_data);
    for i in (inventory_start..file_data.bytes.len()).step_by(16) {
        let index = file_data.bytes[i];
        let id = u32::from_le_bytes([file_data.bytes[i + 8], file_data.bytes[i + 9], file_data.bytes[i + 10], 0]);
        let first_part =
            u32::from_le_bytes([file_data.bytes[i + 4], file_data.bytes[i + 5], file_data.bytes[i + 6], file_data.bytes[i + 7]]);
        let second_part =
            u32::from_le_bytes([file_data.bytes[i + 8], file_data.bytes[i + 9], file_data.bytes[i + 10], file_data.bytes[i + 11]]);
        let amount =
            u32::from_le_bytes([file_data.bytes[i + 12], file_data.bytes[i + 13], file_data.bytes[i + 14], file_data.bytes[i + 15]]);

        if first_part == 0 && second_part == u32::MAX && amount == 0 {
            break;
        }

        
        let info = get_info(id).unwrap();

        let article_type = match (file_data.bytes[i+7],file_data.bytes[i+11]) {
            (0xB0,0x40) => ArticleType::Item,
            (_,0x10) => ArticleType::Armor,
            _ => ArticleType::Weapon,
        };

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

pub fn get_info(id: u32) -> Result<Option<ItemInfo>, Error> {
    let json_file = File::open("items.json").map_err(Error::IoError)?;
    let reader = BufReader::new(json_file);
    let items: Value = serde_json::from_reader(reader).unwrap();

    if let Some(object) = items.as_object() {
        for (_, category) in object {
            if let Some(category) = category.as_object() {
                if let Some((_, value)) = category
                    .iter()
                    .find(|(key, _)| key.parse::<u32>().ok() == Some(id))
                {
                    let item_info: ItemInfo = serde_json::from_value(value.clone()).map_err(Error::JsonError)?;
                    return Ok(Some(item_info));
                }
            }
        }
    }

    if let Some(consumables) = items["consumables"].as_object() {
        for (_, category) in consumables {
            if let Some(category) = category.as_object() {
                if let Some((_, value)) = category
                    .iter()
                    .find(|(key, _)| key.parse::<u32>().ok() == Some(id))
                {
                    let item_info: ItemInfo = serde_json::from_value(value.clone()).map_err(Error::JsonError)?;
                    return Ok(Some(item_info));
                }
            }
        }
    }

    Ok(None)
}
