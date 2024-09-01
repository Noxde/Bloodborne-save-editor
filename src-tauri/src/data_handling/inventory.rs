use serde::{Deserialize, Serialize};
use serde_json::{self, json, Value};
use super::{enums::{Imprint, ArticleType, Error, TypeFamily}, file::FileData};
use std::{fs::File,
          io::BufReader,
          collections::HashMap,
          path::PathBuf};
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ItemInfo {
    pub item_name: String,
    pub item_desc: String,
    pub item_img: String,
    pub extra_info: Option<Value>
}

//Describes the imprint and the upgrade level of a weapon
pub struct WeaponMods {
    upgrade_level: u8,
    imprint: Option<Imprint>,
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
    pub type_family: TypeFamily,
}

impl Article {
    pub fn transform(&mut self, file_data: &mut FileData, new_id: u32) -> Result<(), Error>{
        let mut new_id = new_id.to_le_bytes().to_vec();
        match self.type_family {
            TypeFamily::Item => {
                new_id.pop();
                self.transform_item(file_data, new_id)
            },
            TypeFamily::Armor | TypeFamily::Weapon => self.transform_armor_or_weapon(file_data, new_id),
        }
    }
    fn transform_item(&mut self, file_data: &mut FileData, new_id: Vec<u8>) -> Result<(), Error>{
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
        match file_data.find_article_offset(self.index, self.id, self.type_family) {
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

    fn transform_armor_or_weapon(&mut self, file_data: &mut FileData, new_id: Vec<u8>) -> Result<(), Error>{
        if new_id.len()!=4 {
            return Err(Error::CustomError("ERROR: 'new_id' argument must be 4B long."));
        }

        let i;
        match file_data.find_article_offset(self.index, self.id, self.type_family) {
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
        match file_data.find_article_offset(self.index, self.id, self.type_family) {
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
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Inventory {
    pub articles: HashMap<ArticleType, Vec<Article>>,
}

impl Inventory {
    ///Modifies the amount of an article
    pub fn edit_item(&mut self, file_data: &mut FileData, index: u8, id: u32, value: u32) -> Result<(), Error> {
        let value_endian = u32::to_le_bytes(value);
        let mut found = false;
        for (k, v) in self.articles.iter_mut() {
            let family: TypeFamily = k.to_owned().into();
            if family == TypeFamily::Item {
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

        let opt = file_data.find_article_offset(index, id, TypeFamily::Item);
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

    pub fn add_item(&mut self, file_data: &mut FileData, id: u32, quantity: u32) -> Result<(), Error> {
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
        (file_data.bytes[inventory_end + 12], _) = file_data.bytes[inventory_end - 4].overflowing_add(1);

        let id = u32::from_le_bytes(endian_id);

        if let Ok((info, article_type)) = get_info_item(id, &file_data.resources_path) {
            let new_item = Article {
                index: file_data.bytes[inventory_end + 12],
                id,
                first_part: 213,
                second_part: 1233,
                info,
                amount: quantity,
                article_type,
                type_family: article_type.into(),
            };
            self.articles.entry(article_type).or_insert(Vec::new()).push(new_item);
            return Ok(());
        }
        Err(Error::CustomError("ERROR: failed to find info for the item."))
    }
}

pub fn build(file_data: &FileData) -> Inventory {
    Inventory {
        articles: parse_articles(file_data),
    }
}

pub fn parse_articles(file_data: &FileData) -> HashMap<ArticleType, Vec<Article>> {
    let mut articles = HashMap::new();
    let mut parse = |start: usize, end: usize| {
        for i in (start .. end).step_by(16) {
            let index = file_data.bytes[i];
            let mut id = u32::from_le_bytes([file_data.bytes[i + 8], file_data.bytes[i + 9], file_data.bytes[i + 10], 0]);
            let first_part =
                u32::from_le_bytes([file_data.bytes[i + 4], file_data.bytes[i + 5], file_data.bytes[i + 6], file_data.bytes[i + 7]]);
            let second_part =
                u32::from_le_bytes([file_data.bytes[i + 8], file_data.bytes[i + 9], file_data.bytes[i + 10], file_data.bytes[i + 11]]);
            let amount =
                u32::from_le_bytes([file_data.bytes[i + 12], file_data.bytes[i + 13], file_data.bytes[i + 14], file_data.bytes[i + 15]]);

            let result = match (file_data.bytes[i+7],file_data.bytes[i+11]) {
                (0xB0,0x40) => get_info_item(id, &file_data.resources_path),
                (_,0x10) => get_info_armor(id, &file_data.resources_path),
                _ => {
                    id = second_part;
                    get_info_weapon(id, &file_data.resources_path)
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
                    type_family: article_type.into(),
                };
                let category = articles.entry(article_type).or_insert(Vec::new());
                category.push(article);
            };
        }
    };
    parse(file_data.offsets.inventory.0, file_data.offsets.inventory.1); //parse the inventory
    parse(file_data.offsets.key_inventory.0, file_data.offsets.key_inventory.1); //parse the key inventory
    articles
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
    if id != 12080000 { //Special case
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

pub fn scale_weapon_info(extra_info: &mut Value) {
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

    const TEST_SAVE_PATH: &str = "saves/testsave0";

    fn build_file_data() -> FileData {
        FileData::build(TEST_SAVE_PATH, PathBuf::from("resources")).unwrap()
    }

    fn check_bytes(file_data: &FileData,index: usize,bytes: &[u8]) -> bool {
        let mut equal = true;
        for (i, byte) in bytes.iter().enumerate() {
            if file_data.bytes[index+i]!=*byte {
                equal = false;
                break;
            }
        }
        if equal == false {
            println!("check_bytes failed:");
            for (i, byte) in bytes.iter().enumerate() {
                println!("File byte: {:#02x}, test byte: {:#02x}", file_data.bytes[index+i], *byte);
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
        let result = article.transform(&mut file_data, u32::from_le_bytes([0xAA,0xBB,0xCC,0x00]));
        assert!(result.is_err());
        if let Err(error) = result {
            assert_eq!(error.to_string(), "Save error: ERROR: Failed to find info for the item.");
        }
        article.transform(&mut file_data, u32::from_le_bytes([0x64,0x1B,0x00,0x00])).unwrap();
        assert!(check_bytes(&file_data, 0x89cc,
            &[0x48,0x80,0xCF,0xA8,0x64,0x1B,0x00,0xB0,0x64,0x1B,0x00,0x40,0x01,0,0,0]));
        assert_eq!(article.id, u32::from_le_bytes([0x64,0x1B,0x00,0x00]));
        assert_eq!(article.first_part, u32::from_le_bytes([0x64,0x1B,0x00,0xB0]));
        assert_eq!(article.second_part, u32::from_le_bytes([0x64,0x1B,0x00,0x40]));
        assert_eq!(article.article_type, ArticleType::Material);

        //error tests
        assert!(article.transform_item(&mut file_data, vec![0xAA,0xBB]).is_err());
        assert!(article.transform(&mut file_data, u32::from_le_bytes([0xAA,0xBB,0xCC,0xDD])).is_err());
        article.index = 255;
        let result = article.transform(&mut file_data, u32::from_le_bytes([0x64,0x1B,0x00,0x00]));
        assert!(result.is_err());
        if let Err(error) = result {
            assert_eq!(error.to_string(), "Save error: ERROR: The Article was not found in the inventory.");
        }

        //test transforming a Consumable into a Key
        let article = &mut inventory.articles.get_mut(&ArticleType::Consumable).unwrap()[1];
        assert_eq!(article.amount, 20);
        assert!(check_bytes(&file_data, 0x89FC,
            &[0x4B,0x00,0xFD,0x7F,0x84,0x03,0x00,0xB0,0x84,0x03,0x00,0x40,0x14,0x00,0x00,0x00]));
        article.transform(&mut file_data, u32::from_le_bytes([0xA0,0x0F,0x00,0x00])).unwrap();
        assert!(check_bytes(&file_data, 0x89FC,
            &[0x4B,0x00,0xFD,0x7F,0xA0,0x0F,0x00,0xB0,0xA0,0x0F,0x00,0x40,0x01,0x00,0x00,0x00]));
        assert_eq!(article.article_type, ArticleType::Key);
        assert_eq!(article.amount, 1);

        //Transform an item in the key inventory
        let article = &mut inventory.articles.get_mut(&ArticleType::Key).unwrap()[2];
        assert_eq!(article.id, u32::from_le_bytes([0xd8, 0x10, 0x00,0x00]));
        assert_eq!(article.amount, 1);
        assert!(check_bytes(&file_data, 0x10550,
            &[0x00,0xc0,0x07,0xa6,0xd8,0x10,0x00,0xb0,0xd8,0x10,0x00,0x40,0x01,0x00,0x00,0x00]));
        article.transform(&mut file_data, u32::from_le_bytes([0xea,0x10,0x00,0x00])).unwrap();
        assert!(check_bytes(&file_data, 0x10550,
            &[0x00,0xc0,0x07,0xa6,0xea,0x10,0x00,0xb0,0xea,0x10,0x00,0x40,0x01,0x00,0x00,0x00]));
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
        let result = article.transform(&mut file_data, u32::from_le_bytes([0xAA,0xBB,0xCC,0xDD]));
        assert!(result.is_err());
        if let Err(error) = result {
            assert_eq!(error.to_string(), "Save error: ERROR: Failed to find info for the article.");
        }
        article.transform(&mut file_data, u32::from_le_bytes([0x40,0x4b,0x4c,0x00])).unwrap();

        assert!(check_bytes(&file_data, 0x89ec,
            &[0x4a,0x00,0x83,0x7c,0x51,0x00,0x80,0x80,0x40,0x4b,0x4c,0x00,0x01,0x00,0x00,0x00]));
        assert!(check_bytes(&file_data, 0x5f8,
            &[0x40,0x4b,0x4c,0x00]));
        assert_eq!(article.id, u32::from_le_bytes([0x40,0x4b,0x4c,0x00]));
        assert_eq!(article.first_part, u32::from_le_bytes([0x51,0x00,0x80,0x80]));
        assert_eq!(article.second_part, u32::from_le_bytes([0x40,0x4b,0x4c,0x00]));

        //transform an armor
        let article = &mut inventory.articles.get_mut(&ArticleType::Armor).unwrap()[0];
        assert!(check_bytes(&file_data, 0x898c,
            &[0x44,0xf0,0xff,0xff,0x48,0x00,0x80,0x90,0x70,0x82,0x03,0x10,0x01,0x00,0x00,0x00]));
        assert!(check_bytes(&file_data, 0x3dc,
            &[0x70,0x82,0x03,0x10]));

        article.transform(&mut file_data, u32::from_le_bytes([0x60,0x5b,0x03,0x00])).unwrap();

        assert!(check_bytes(&file_data, 0x898c,
            &[0x44,0xf0,0xff,0xff,0x48,0x00,0x80,0x90,0x60,0x5b,0x03,0x10,0x01,0x00,0x00,0x00]));
        assert!(check_bytes(&file_data, 0x3dc,
            &[0x60,0x5b,0x03,0x10]));
        assert_eq!(article.id, u32::from_le_bytes([0x60,0x5b,0x03,0x00]));
        assert_eq!(article.first_part, u32::from_le_bytes([0x48,0x00,0x80,0x90]));
        assert_eq!(article.second_part, u32::from_le_bytes([0x60,0x5b,0x03,0x10]));

        //error tests
        assert!(article.transform_armor_or_weapon(&mut file_data, vec![0xAA,0xBB,0xCC]).is_err());
        assert!(article.transform_armor_or_weapon(&mut file_data, vec![0xAA,0xBB,0xCC,0xDD,0xEE]).is_err());

        article.index = 255;
        let result = article.transform_armor_or_weapon(&mut file_data, vec![0xAA,0xBB,0xCC,0xDD]);
        assert!(result.is_err());
        if let Err(error) = result {
            assert_eq!(error.to_string(), "Save error: ERROR: The Article was not found in the inventory.");
        }
    }

    #[test]
    fn inventory_edit_item() {
        let mut file_data = build_file_data();
        let mut inventory = build(&file_data);
        assert!(check_bytes(&file_data, 0x89cc, 
            &[0x48,0x80,0xCF,0xA8,0x64,0,0,0xB0,0x64,0,0,0x40,0x01,0,0,0]));
        //Try to edit a key item
        let result = inventory.edit_item(&mut file_data, 0x00, 0xAAAAAAAA, 0xAABBCCDD);
        assert!(result.is_err());
        if let Err(error) = result {
            assert_eq!(error.to_string(), "Save error: ERROR: Key items cannot be edited.");
        }
        //Try wrong index
        let result = inventory.edit_item(&mut file_data, 0xAA, 0xAAAAAAAA, 0xAABBCCDD);
        assert!(result.is_err());
        if let Err(error) = result {
            assert_eq!(error.to_string(), "Save error: ERROR: The Article was not found in the inventory.");
        }

        inventory.edit_item(&mut file_data, 0x48, 0x64, 0xAABBCCDD).unwrap();
        assert!(check_bytes(&file_data, 0x89cc, 
            &[0x48,0x80,0xCF,0xA8,0x64,0,0,0xB0,0x64,0,0,0x40,0xDD,0xCC,0xBB,0xAA]));
        assert_eq!(inventory.articles.get(&ArticleType::Consumable).unwrap()[0].amount, 0xAABBCCDD);
    }

    #[test]
    fn test_parse_key_inventory() {
        let file_data = FileData::build("saves/testsave0", PathBuf::from("resources")).unwrap();
        let articles = parse_articles(&file_data);
        let keys = articles.get(&ArticleType::Key).unwrap();
        assert_eq!(keys.len(), 7);

        //Item N0
        assert_eq!(keys[0].index, 0x5e);
        assert_eq!(keys[0].id, u32::from_le_bytes([0xd2, 0x10, 0x00, 0x00]));
        assert_eq!(keys[0].first_part, u32::from_le_bytes([0xd2, 0x10, 0x00, 0xb0]));
        assert_eq!(keys[0].second_part, u32::from_le_bytes([0xd2, 0x10, 0x00, 0x40]));
        assert_eq!(keys[0].amount, u32::from_le_bytes([0x01, 0x00, 0x00, 0x00]));
        assert_eq!(keys[0].article_type, ArticleType::Key);

        //Item N1
        assert_eq!(keys[1].index, 6);
        assert_eq!(keys[1].id, u32::from_le_bytes([0x12, 0x10, 0x00, 0x00]));
        assert_eq!(keys[1].first_part, u32::from_le_bytes([0x12, 0x10, 0x00, 0xb0]));
        assert_eq!(keys[1].second_part, u32::from_le_bytes([0x12, 0x10, 0x00, 0x40]));
        assert_eq!(keys[1].amount, u32::from_le_bytes([0x01, 0x00, 0x00, 0x00]));
        assert_eq!(keys[1].article_type, ArticleType::Key);

        //Item N2
        assert_eq!(keys[2].index, 0);
        assert_eq!(keys[2].id, u32::from_le_bytes([0xd8, 0x10, 0x00, 0x00]));
        assert_eq!(keys[2].first_part, u32::from_le_bytes([0xd8, 0x10, 0x00, 0xb0]));
        assert_eq!(keys[2].second_part, u32::from_le_bytes([0xd8, 0x10, 0x00, 0x40]));
        assert_eq!(keys[2].amount, u32::from_le_bytes([0x01, 0x00, 0x00, 0x00]));
        assert_eq!(keys[2].article_type, ArticleType::Key);

        //Item N3
        assert_eq!(keys[3].index, 1);
        assert_eq!(keys[3].id, u32::from_le_bytes([0x0e, 0x10, 0x00, 0x00]));
        assert_eq!(keys[3].first_part, u32::from_le_bytes([0x0e, 0x10, 0x00, 0xb0]));
        assert_eq!(keys[3].second_part, u32::from_le_bytes([0x0e, 0x10, 0x00, 0x40]));
        assert_eq!(keys[3].amount, u32::from_le_bytes([0x01, 0x00, 0x00, 0x00]));
        assert_eq!(keys[3].article_type, ArticleType::Key);

        //Item N4
        assert_eq!(keys[4].index, 2);
        assert_eq!(keys[4].id, u32::from_le_bytes([0xa0, 0x0f, 0x00, 0x00]));
        assert_eq!(keys[4].first_part, u32::from_le_bytes([0xa0, 0x0f, 0x00, 0xb0]));
        assert_eq!(keys[4].second_part, u32::from_le_bytes([0xa0, 0x0f, 0x00, 0x40]));
        assert_eq!(keys[4].amount, u32::from_le_bytes([0x01, 0x00, 0x00, 0x00]));
        assert_eq!(keys[4].article_type, ArticleType::Key);

        //Item N5
        assert_eq!(keys[5].index, 3);
        assert_eq!(keys[5].id, u32::from_le_bytes([0x07, 0x10, 0x00, 0x00]));
        assert_eq!(keys[5].first_part, u32::from_le_bytes([0x07, 0x10, 0x00, 0xb0]));
        assert_eq!(keys[5].second_part, u32::from_le_bytes([0x07, 0x10, 0x00, 0x40]));
        assert_eq!(keys[5].amount, u32::from_le_bytes([0x01, 0x00, 0x00, 0x00]));
        assert_eq!(keys[5].article_type, ArticleType::Key);

        //Item N6
        assert_eq!(keys[6].index, 4);
        assert_eq!(keys[6].id, u32::from_le_bytes([0xab, 0x0f, 0x00, 0x00]));
        assert_eq!(keys[6].first_part, u32::from_le_bytes([0xab, 0x0f, 0x00, 0xb0]));
        assert_eq!(keys[6].second_part, u32::from_le_bytes([0xab, 0x0f, 0x00, 0x40]));
        assert_eq!(keys[6].amount, u32::from_le_bytes([0x01, 0x00, 0x00, 0x00]));
        assert_eq!(keys[6].article_type, ArticleType::Key);
    }

    #[test]
    fn inventory_add_item() {
        let mut file_data = build_file_data();
        let mut inventory = build(&file_data);
        assert_eq!(inventory.articles.get(&ArticleType::Consumable).unwrap().len(), 15);
        assert!(check_bytes(&file_data, 0x8cdb, 
            &[0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0xff,0xff,0xff,0xff,0x00,0x00,0x00]));
        //Try to add an invalid item
        let result = inventory.add_item(&mut file_data, 0x00, 0x00);
        assert!(result.is_err());
        if let Err(error) = result {
            assert_eq!(error.to_string(), "Save error: ERROR: failed to find info for the item.");
        }

        inventory.add_item(&mut file_data, u32::from_le_bytes([0x60, 0x04, 0x00, 0x00]), 32).unwrap();
        assert_eq!(inventory.articles.get(&ArticleType::Consumable).unwrap().len(), 16);
        assert!(check_bytes(&file_data, 0x8cdb, 
            &[0x60,0x04,0x00,0xb0,0x60,0x04,0x00,0x40,0x20,0x00,0x00,0x00,0x00,0x00,0x00,0x00]));

        //let file_data = build_file_data();
        //let inventory = build(&file_data);
        //let consumables = inventory.articles.get(&ArticleType::Consumable).unwrap();
        //let new_item = consumables.last().unwrap();
        //assert_eq!(consumables.len(), 16);
        //assert_eq!(new_item.index, 5);
        //assert_eq!(new_item.id, u32::from_le_bytes([0x60, 0x04, 0x00, 0x00]));
        //assert_eq!(new_item.first_part, u32::from_le_bytes([0x60, 0x04, 0x00, 0xb0]));
        //assert_eq!(new_item.second_part, u32::from_le_bytes([0x60, 0x04, 0x00, 0x40]));
        //assert_eq!(new_item.amount, u32::from_le_bytes([0x20, 0x00, 0x00, 0x00]));
        //assert_eq!(new_item.article_type, ArticleType::Consumable);
    }

    #[test]
    fn article_is_type_family() {
        let file_data = build_file_data();
        let inventory = build(&file_data);
        let armor = inventory.articles.get(&ArticleType::Armor).unwrap()[0].clone();
        let weapon = inventory.articles.get(&ArticleType::LeftHand).unwrap()[0].clone();
        let item = inventory.articles.get(&ArticleType::Material).unwrap()[0].clone();
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
        let file_data = FileData::build("saves/weaponmods0", PathBuf::from("resources")).unwrap();
        let articles = parse_articles(&file_data);
        let right_hands = articles.get(&ArticleType::RightHand).unwrap();
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
        let mut file_data = build_file_data();
        let mut inventory = build(&file_data);

        let article = &mut inventory.articles.get_mut(&ArticleType::LeftHand).unwrap()[0];
        assert!(check_bytes(&file_data, 0x89ec,
            &[0x4a,0x00,0x83,0x7c,0x51,0x00,0x80,0x80,0x80,0x9f,0xd5,0x00,0x01,0x00,0x00,0x00]));
        assert!(check_bytes(&file_data, 0x5f8,
            &[0x80,0x9f,0xd5,0x00]));
        let extra_info = article.info.extra_info.clone().unwrap();
        assert_eq!(extra_info["imprint"], Value::Null);
        assert_eq!(extra_info["upgrade_level"], Value::from(0));
        // 0xD59F80 + 20800 = 0xD5F0C0
        article.set_imprint_and_upgrade(&mut file_data, Some(Some(Imprint::Lost)), Some(8)).unwrap();
        assert!(check_bytes(&file_data, 0x89ec,
            &[0x4a,0x00,0x83,0x7c,0x51,0x00,0x80,0x80,0xC0,0xF0,0xD5,0x00,0x01,0x00,0x00,0x00]));
        assert!(check_bytes(&file_data, 0x5f8,
            &[0xC0,0xF0,0xD5,0x00]));
        assert_eq!(article.id, u32::from_le_bytes([0xC0,0xF0,0xD5,0x00]));
        assert_eq!(article.first_part, u32::from_le_bytes([0x51,0x00,0x80,0x80]));
        assert_eq!(article.second_part, u32::from_le_bytes([0xC0,0xF0,0xD5,0x00]));
        let extra_info = article.info.extra_info.clone().unwrap();
        assert_eq!(extra_info["imprint"], json!(Some(Imprint::Lost)));
        assert_eq!(extra_info["upgrade_level"], Value::from(8));

        //Test errors
        let article = &mut inventory.articles.get_mut(&ArticleType::Armor).unwrap()[0];
        let result = article.set_imprint_and_upgrade(&mut file_data, None, None);
        assert!(result.is_err());
        if let Err(error) = result {
            assert_eq!(error.to_string(), "Save error: ERROR: The article must be a weapon.");
        }

        let article = &mut inventory.articles.get_mut(&ArticleType::RightHand).unwrap()[0];
        let result = article.set_imprint_and_upgrade(&mut file_data, None, Some(11));
        assert!(result.is_err());
        if let Err(error) = result {
            assert_eq!(error.to_string(), "Save error: ERROR: Upgrade level cannot be bigger than 10.");
        }

        article.info.extra_info = None;
        let result = article.set_imprint_and_upgrade(&mut file_data, None, None);
        assert!(result.is_err());
        if let Err(error) = result {
            assert_eq!(error.to_string(), "Save error: ERROR: The weapon has no extrainfo.");
        }

        article.second_part = u32::MAX;
        let result = article.set_imprint_and_upgrade(&mut file_data, None, None);
        assert!(result.is_err());
        if let Err(error) = result {
            assert_eq!(error.to_string(), "Save error: ERROR: Invalid second_part");
        }

        let article = &mut inventory.articles.get_mut(&ArticleType::RightHand).unwrap()[1];
        article.index = 0xEE;
        let result = article.set_imprint_and_upgrade(&mut file_data, None, None);
        assert!(result.is_err());
        if let Err(error) = result {
            assert_eq!(error.to_string(), "Save error: ERROR: The Article was not found in the inventory.");
        }
    }
}
