use serde::Deserialize;
use serde_json::{self, Value};

use std::{error::Error, fs::File, io::BufReader};

#[derive(Deserialize, Debug)]
pub struct ItemInfo {
    pub item_name: String,
    pub item_desc: String,
    pub item_img: String,
}

#[derive(Debug)]
pub struct Item {
    // pub name: String,
    pub index: u8,
    pub id: u32,
    pub first_part: u32,
    pub second_part: u32,
    pub amount: u32,
    pub info: Option<ItemInfo>,
}

pub struct Inventory {
    pub items: Vec<Item>,
}

impl Inventory {
    pub fn _edit_item(&mut self, bytes: &mut Vec<u8>, index: u8, value: u32) {
        let value_endian = u32::to_le_bytes(value);
        let (start, _) = inventory_offset(bytes);

        for i in (start..bytes.len()).step_by(16) {
            if index == bytes[i] {
                for (i, b) in bytes[i + 12..i + 16].iter_mut().enumerate() {
                    if let Some(item) = self.items.iter_mut().find(|item| item.index == index) {
                        item.amount = value;
                    }
                    *b = value_endian[i];
                }
                break;
            }
        }
    }

    pub fn _add_item(&mut self, bytes: &mut Vec<u8>, id: u32, quantity: u32) {
        let (_, inventory_end) = inventory_offset(bytes);
        let endian_id = u32::to_le_bytes(id);
        let endian_quantity = u32::to_le_bytes(quantity);

        for i in 0..12 {
            if i < 8 {
                bytes[inventory_end + i] = endian_id[i % 4];
            } else {
                bytes[inventory_end + i] = endian_quantity[i % 4];
            }
        }
        bytes[inventory_end + 3] = 0xB0;
        bytes[inventory_end + 7] = 0x40;
        bytes[inventory_end + 12] = bytes[inventory_end - 4] + 1;

        let new_item = Item {
            index: bytes[inventory_end + 12],
            id: u32::from_le_bytes(endian_id),
            first_part: 213,
            second_part: 1233,
            info: get_info(id).unwrap(),
            amount: quantity,
        };

        self.items.push(new_item);
    }
}

pub fn build(bytes: &Vec<u8>) -> Inventory {
    Inventory {
        items: parse_items(bytes),
    }
}

pub fn inventory_offset(bytes: &[u8]) -> (usize, usize) {
    let mut matches: (usize, usize) = (0, 0);
    for i in 0..bytes.len() - 4 {
        let mut buffer = [0; 4];
        buffer[..4].copy_from_slice(&bytes[i..i + 4]);
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

pub fn parse_items(bytes: &[u8]) -> Vec<Item> {
    let mut items = Vec::new();
    let (inventory_start, _) = inventory_offset(bytes);

    for i in (inventory_start..bytes.len()).step_by(16) {
        let index = bytes[i];
        let id = u32::from_le_bytes([bytes[i + 8], bytes[i + 9], bytes[i + 10], 0]);
        let first_part =
            u32::from_le_bytes([bytes[i + 4], bytes[i + 5], bytes[i + 6], bytes[i + 7]]);
        let second_part =
            u32::from_le_bytes([bytes[i + 8], bytes[i + 9], bytes[i + 10], bytes[i + 11]]);
        let amount =
            u32::from_le_bytes([bytes[i + 12], bytes[i + 13], bytes[i + 14], bytes[i + 15]]);

        if first_part == 0 && second_part == u32::MAX && amount == 0 {
            break;
        }

        if bytes[i + 7] != 0xB0 || bytes[i + 11] != 0x40 {
            continue;
        }

        let info = get_info(id).unwrap();

        items.push(Item {
            index,
            id,
            first_part,
            second_part,
            amount,
            info,
        });
    }

    items
}

pub fn get_info(id: u32) -> Result<Option<ItemInfo>, Box<dyn Error>> {
    let json_file = File::open("items.json")?;
    let reader = BufReader::new(json_file);
    let items: Value = serde_json::from_reader(reader).unwrap();

    if let Some(object) = items.as_object() {
        for (_, category) in object {
            if let Some(category) = category.as_object() {
                if let Some((_, value)) = category
                    .iter()
                    .find(|(key, _)| key.parse::<u32>().ok() == Some(id))
                {
                    let item_info: ItemInfo = serde_json::from_value(value.clone())?;
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
                    let item_info: ItemInfo = serde_json::from_value(value.clone())?;
                    return Ok(Some(item_info));
                }
            }
        }
    }

    Ok(None)
}
