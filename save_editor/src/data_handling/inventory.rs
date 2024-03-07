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

pub fn build(bytes: &[u8]) -> Inventory {
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
        if t == current {
            if matches.0 == 0 {
                matches.0 = i;
            } else {
                matches.1 = i;
                break;
            }
        }
    }
    matches
}

pub fn parse_items(bytes: &[u8]) -> Vec<Item> {
    let mut items = Vec::new();
    let (inventory, _storage) = inventory_offset(bytes);
    println!("{inventory}");

    for i in (inventory..bytes.len()).step_by(16) {
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

    if let Some((_, value)) = items
        .as_object()
        .ok_or("Items is not an object")?
        .iter()
        .find(|(key, _)| key.parse::<u32>().ok() == Some(id))
    {
        let item_info: ItemInfo = serde_json::from_value(value.clone())?;
        return Ok(Some(item_info));
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
