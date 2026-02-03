use super::{
    article::{scale_weapon_info, Article, ItemInfo, WeaponMods},
    constants::*,
    enums::{ArticleType, Error, Location, TypeFamily, UpgradeType},
    file::FileData,
    slots::Slot,
    upgrades::Upgrade,
};
use serde::{Deserialize, Serialize};
use serde_json::{self, json, Value};
use std::{collections::HashMap, fs::File, io::BufReader, path::PathBuf};

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
    pub fn build(
        file_data: &FileData,
        inv: (usize, usize),
        key: (usize, usize),
        all_upgrades: &mut HashMap<u32, (Upgrade, UpgradeType)>,
        all_slots: &mut HashMap<u64, Vec<Slot>>,
    ) -> Inventory {
        let mut has_slots = false;
        let mut articles = HashMap::new();
        let mut upgrades = HashMap::new();
        let mut first_upgrade = None;
        let mut first_article = None;
        let mut first = true;

        let mut parse = |start: usize, end: usize| {
            for (index, i) in (start..end).step_by(16).enumerate() {
                let number = file_data.bytes[i];
                let mut id = u32::from_le_bytes([
                    file_data.bytes[i + 8],
                    file_data.bytes[i + 9],
                    file_data.bytes[i + 10],
                    0,
                ]);
                let first_part = u32::from_le_bytes([
                    file_data.bytes[i + 4],
                    file_data.bytes[i + 5],
                    file_data.bytes[i + 6],
                    file_data.bytes[i + 7],
                ]);
                let second_part = u32::from_le_bytes([
                    file_data.bytes[i + 8],
                    file_data.bytes[i + 9],
                    file_data.bytes[i + 10],
                    file_data.bytes[i + 11],
                ]);
                let amount = u32::from_le_bytes([
                    file_data.bytes[i + 12],
                    file_data.bytes[i + 13],
                    file_data.bytes[i + 14],
                    file_data.bytes[i + 15],
                ]);

                let result = match (file_data.bytes[i + 7], file_data.bytes[i + 11]) {
                    (0xB0, 0x40) => {
                        has_slots = false;
                        get_info_item(id, &file_data.resources_path)
                    }
                    (_, 0x10) => {
                        has_slots = true;
                        get_info_armor(id, &file_data.resources_path)
                    }
                    _ => {
                        id = second_part;
                        has_slots = true;
                        get_info_weapon(id, &file_data.resources_path)
                    }
                };

                if let Ok((info, article_type)) = result {
                    let mut slots = None;
                    if has_slots {
                        let key = u64::from_le_bytes([
                            file_data.bytes[i + 4],
                            file_data.bytes[i + 5],
                            file_data.bytes[i + 6],
                            file_data.bytes[i + 7],
                            file_data.bytes[i + 8],
                            file_data.bytes[i + 9],
                            file_data.bytes[i + 10],
                            file_data.bytes[i + 11],
                        ]);
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
                } else if let Some(mut upgrade) = all_upgrades.remove(&first_part) {
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
    pub fn edit_item(
        &mut self,
        file_data: &mut FileData,
        number: u8,
        id: u32,
        value: u32,
        is_storage: bool,
    ) -> Result<(), Error> {
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
            for (i, o) in (offset + 12..offset + 16).enumerate() {
                file_data.bytes[o] = value_endian[i];
            }
        }

        if opt.is_none() || !found {
            return Err(Error::CustomError(
                "ERROR: The Article was not found in the inventory.",
            ));
        }

        Ok(())
    }

    pub fn add_item(
        &mut self,
        file_data: &mut FileData,
        id: u32,
        quantity: u32,
        is_storage: bool,
    ) -> Result<&mut Inventory, Error> {
        let result = get_info_item(id, &file_data.resources_path);
        if result.is_err() {
            return Err(Error::CustomError(
                "ERROR: failed to find info for the item.",
            ));
        }

        let empty_slot_index;
        match file_data.find_inv_empty_slot(Location::from(is_storage)) {
            Some(index) => empty_slot_index = index,
            None => {
                if !is_storage {
                    empty_slot_index = file_data.offsets.inventory.1;
                    file_data.offsets.inventory.1 += 16;
                } else {
                    empty_slot_index = file_data.offsets.storage.1;
                    file_data.offsets.storage.1 += 16;
                }
                (file_data.bytes[empty_slot_index + 12], _) =
                    file_data.bytes[empty_slot_index - 4].overflowing_add(1);
            }
        };

        let uname = file_data.offsets.username;
        let (first_counter_index, second_counter_index) = {
            if !is_storage {
                (
                    uname + USERNAME_TO_FIRST_INVENTORY_COUNTER,
                    uname + USERNAME_TO_SECOND_INVENTORY_COUNTER,
                )
            } else {
                (
                    uname + USERNAME_TO_FIRST_STORAGE_COUNTER,
                    uname + USERNAME_TO_SECOND_STORAGE_COUNTER,
                )
            }
        };

        let endian_id = u32::to_le_bytes(id);
        let endian_quantity = u32::to_le_bytes(quantity);

        for i in 0..12 {
            if i < 8 {
                file_data.bytes[empty_slot_index + i] = endian_id[i % 4];
            } else {
                file_data.bytes[empty_slot_index + i] = endian_quantity[i % 4];
            }
        }
        file_data.bytes[empty_slot_index + 3] = 0xB0;
        file_data.bytes[empty_slot_index + 7] = 0x40;

        let id = u32::from_le_bytes(endian_id);

        // Create the first_part array
        let mut first_part = [0u8; 4];
        first_part[..endian_id.len()].copy_from_slice(&endian_id);
        first_part[first_part.len() - 1] = 0xB0;

        // Create the second_part array
        let mut second_part = [0u8; 4];
        second_part[..endian_id.len()].copy_from_slice(&endian_id);
        second_part[second_part.len() - 1] = 0x40;

        let new_counter_value = u32::from_le_bytes([
            file_data.bytes[first_counter_index],
            file_data.bytes[first_counter_index + 1],
            file_data.bytes[first_counter_index + 2],
            file_data.bytes[first_counter_index + 3],
        ]) + 1;
        let new_counter_value_bytes: [u8; 4] = new_counter_value.to_le_bytes();
        for i in 0..4 {
            file_data.bytes[i + first_counter_index] = new_counter_value_bytes[i];
        }

        let new_counter_value = u32::from_le_bytes([
            file_data.bytes[second_counter_index],
            file_data.bytes[second_counter_index + 1],
            file_data.bytes[second_counter_index + 2],
            file_data.bytes[second_counter_index + 3],
        ]) + 1;
        let new_counter_value_bytes: [u8; 4] = new_counter_value.to_le_bytes();
        for i in 0..4 {
            file_data.bytes[i + second_counter_index] = new_counter_value_bytes[i];
        }

        let (info, article_type) = result.expect("Err variant checked at the beginning");

        let mut new_item = Article {
            number: file_data.bytes[empty_slot_index - 4],
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
                new_item.number = file_data.bytes[first_counter_index];
            }
        }

        let vec = self.articles.entry(article_type).or_insert(Vec::new());
        new_item.index = vec.len();
        vec.push(new_item);

        return Ok(self);
    }

    //This method asumes that upgrade exists in file_data and it's not in the inventory
    pub fn add_upgrade(
        &mut self,
        file_data: &mut FileData,
        mut upgrade: Upgrade,
        is_storage: bool,
    ) {
        let empty_slot_index;
        match file_data.find_inv_empty_slot(Location::from(is_storage)) {
            Some(index) => empty_slot_index = index,
            None => {
                if !is_storage {
                    empty_slot_index = file_data.offsets.inventory.1;
                    file_data.offsets.inventory.1 += 16;
                } else {
                    empty_slot_index = file_data.offsets.storage.1;
                    file_data.offsets.storage.1 += 16;
                }
                (file_data.bytes[empty_slot_index + 12], _) =
                    file_data.bytes[empty_slot_index - 4].overflowing_add(1);
            }
        };
        let uname = file_data.offsets.username;
        let (first_counter_index, second_counter_index) = {
            if !is_storage {
                (
                    uname + USERNAME_TO_FIRST_INVENTORY_COUNTER,
                    uname + USERNAME_TO_SECOND_INVENTORY_COUNTER,
                )
            } else {
                (
                    uname + USERNAME_TO_FIRST_STORAGE_COUNTER,
                    uname + USERNAME_TO_SECOND_STORAGE_COUNTER,
                )
            }
        };

        let endian_id = u32::to_le_bytes(upgrade.id);
        let endian_source = u32::to_le_bytes(upgrade.source);
        let endian_quantity = [0x01, 0x00, 0x00, 0x00];

        for i in 0..4 {
            file_data.bytes[empty_slot_index + i] = endian_id[i];
        }
        for i in 4..8 {
            file_data.bytes[empty_slot_index + i] = endian_source[i % 4];
        }
        for i in 8..12 {
            file_data.bytes[empty_slot_index + i] = endian_quantity[i % 4];
        }

        let new_counter_value = u32::from_le_bytes([
            file_data.bytes[first_counter_index],
            file_data.bytes[first_counter_index + 1],
            file_data.bytes[first_counter_index + 2],
            file_data.bytes[first_counter_index + 3],
        ]) + 1;
        let new_counter_value_bytes: [u8; 4] = new_counter_value.to_le_bytes();
        for i in 0..4 {
            file_data.bytes[i + first_counter_index] = new_counter_value_bytes[i];
        }

        let new_counter_value = u32::from_le_bytes([
            file_data.bytes[second_counter_index],
            file_data.bytes[second_counter_index + 1],
            file_data.bytes[second_counter_index + 2],
            file_data.bytes[second_counter_index + 3],
        ]) + 1;
        let new_counter_value_bytes: [u8; 4] = new_counter_value.to_le_bytes();
        for i in 0..4 {
            file_data.bytes[i + second_counter_index] = new_counter_value_bytes[i];
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
                upgrade.number = file_data.bytes[first_counter_index];
            }
        }

        let vec = self
            .upgrades
            .entry(upgrade.upgrade_type)
            .or_insert(Vec::new());
        upgrade.index = vec.len();
        vec.push(upgrade);
    }

    //This method asumes that the upgrade it's not in the inventory already
    pub fn unequip_gem(
        &mut self,
        file_data: &mut FileData,
        article_type: ArticleType,
        article_index: usize,
        slot_index: usize,
        is_storage: bool,
    ) -> Result<(), Error> {
        if let Some(articles_of_type) = self.articles.get_mut(&article_type) {
            if let Some(article) = articles_of_type.get_mut(article_index) {
                if let Some(ref mut slots) = &mut article.slots {
                    if let Some(slot) = slots.get_mut(slot_index) {
                        if let Some(ref mut gem) = &mut slot.gem {
                            //Remove the gem in file_data
                            let first_part = article.first_part.to_le_bytes();
                            let second_part = article.second_part.to_le_bytes();
                            let mut found = false;
                            for i in
                                file_data.offsets.equipped_gems.0..file_data.offsets.equipped_gems.1
                            {
                                if (file_data.bytes[i..i + 4] == first_part)
                                    && (file_data.bytes[i + 4..i + 8] == second_part)
                                {
                                    found = true;
                                    //24 is the index for the first gem id
                                    let slot_index = i + 24 + 8 * slot_index;
                                    for j in slot_index..slot_index + 4 {
                                        file_data.bytes[j] = 0;
                                    }
                                    break;
                                }
                            }
                            if !found {
                                return Err(Error::CustomError(
                                    "ERROR: Failed to find the article in the file data.",
                                ));
                            }

                            //Remove the gem
                            let gem = gem.to_owned();
                            slot.gem = None;

                            self.add_upgrade(file_data, gem, is_storage);
                            return Ok(());
                        } else {
                            Err(Error::CustomError(
                                "ERROR: The specified slot does not have a gem.",
                            ))
                        }
                    } else {
                        Err(Error::CustomError("ERROR: slot_index is invalid."))
                    }
                } else {
                    Err(Error::CustomError("ERROR: The article has no slots."))
                }
            } else {
                Err(Error::CustomError("ERROR: article_index is invalid."))
            }
        } else {
            Err(Error::CustomError(
                "ERROR: There are no articles of the specified type.",
            ))
        }
    }

    pub fn remove_upgrade(
        &mut self,
        file_data: &mut FileData,
        upgrade_type: UpgradeType,
        upgrade_index: usize,
        is_storage: bool,
    ) -> Result<Upgrade, Error> {
        if let Some(upgrades_of_type) = self.upgrades.get_mut(&upgrade_type) {
            if upgrade_index < upgrades_of_type.len() {
                //Remove the upgrade from the inventory
                let first_part = upgrades_of_type[upgrade_index].id.to_le_bytes();
                let second_part = upgrades_of_type[upgrade_index].source.to_le_bytes();
                let mut found = false;
                let (start, end) = match is_storage {
                    true => file_data.offsets.storage,
                    false => file_data.offsets.inventory,
                };
                //Leave the first 4 bytes
                let empty_slot = [0, 0, 0, 0, 255, 255, 255, 255, 0, 0, 0, 0];
                for i in (start..end).step_by(16) {
                    if (file_data.bytes[i + 4..i + 8] == first_part)
                        && (file_data.bytes[i + 8..i + 12] == second_part)
                    {
                        found = true;
                        for j in i + 4..i + 16 {
                            file_data.bytes[j] = empty_slot[j - i - 4];
                        }
                        break;
                    }
                }
                if !found {
                    return Err(Error::CustomError(
                        "ERROR: Failed to find the upgrade in the inventory.",
                    ));
                }
                //Update the index of the upgrades after the one to be removed
                for i in upgrade_index + 1..upgrades_of_type.len() {
                    upgrades_of_type[i].index -= 1;
                }
                Ok(upgrades_of_type.remove(upgrade_index))
            } else {
                Err(Error::CustomError("ERROR: upgrade_index is invalid."))
            }
        } else {
            Err(Error::CustomError(
                "ERROR: There are no upgrades of the specified type.",
            ))
        }
    }

    pub fn equip_gem(
        &mut self,
        file_data: &mut FileData,
        upgrade_index: usize,
        article_type: ArticleType,
        article_index: usize,
        slot_index: usize,
        is_storage: bool,
    ) -> Result<(), Error> {
        if let Some(articles_of_type) = self.articles.get_mut(&article_type) {
            if let Some(article) = articles_of_type.get_mut(article_index) {
                if let Some(ref mut slots) = &mut article.slots {
                    if let Some(slot) = slots.get_mut(slot_index) {
                        if slot.gem.is_some() {
                            return Err(Error::CustomError(
                                "ERROR: The specified slot already has a gem.",
                            ));
                        }
                        let slot_raw_pointer = slot as *mut Slot;

                        //Remove the gem in file_data
                        let first_part = article.first_part.to_le_bytes();
                        let second_part = article.second_part.to_le_bytes();
                        let mut found = false;
                        for i in
                            file_data.offsets.equipped_gems.0..file_data.offsets.equipped_gems.1
                        {
                            if (file_data.bytes[i..i + 4] == first_part)
                                && (file_data.bytes[i + 4..i + 8] == second_part)
                            {
                                let id_bytes;
                                //Try to get the gem to be equipped
                                let result = self.remove_upgrade(
                                    file_data,
                                    UpgradeType::Gem,
                                    upgrade_index,
                                    is_storage,
                                );

                                match result {
                                    Ok(gem) => {
                                        id_bytes = gem.id.to_le_bytes();
                                        unsafe {
                                            (*slot_raw_pointer).gem = Some(gem);
                                        }
                                    }
                                    Err(error) => return Err(error),
                                };

                                //Equip the gem in the slot
                                found = true;
                                //24 is the index for the first gem id
                                let slot_index = i + 24 + 8 * slot_index;
                                for j in slot_index..slot_index + 4 {
                                    file_data.bytes[j] = id_bytes[j - slot_index];
                                }
                                break;
                            }
                        }
                        if !found {
                            return Err(Error::CustomError(
                                "ERROR: Failed to find the article in the file data.",
                            ));
                        }

                        return Ok(());
                    } else {
                        Err(Error::CustomError("ERROR: slot_index is invalid."))
                    }
                } else {
                    Err(Error::CustomError("ERROR: The article has no slots."))
                }
            } else {
                Err(Error::CustomError("ERROR: article_index is invalid."))
            }
        } else {
            Err(Error::CustomError(
                "ERROR: There are no articles of the specified type.",
            ))
        }
    }

    pub fn change_weapon_level(
        &mut self, 
        file_data: &mut FileData, 
        article_type: ArticleType, 
        article_index: usize, 
        slot_index: usize, 
        is_storage: bool,
        level: u8
    ) -> Result<(), Error> {
        if let Some(articles_of_type) = self.articles.get_mut(&article_type) {
            if let Some(article) = articles_of_type.get_mut(article_index) {
                article.set_imprint_and_upgrade(
                    file_data,
                    None,
                    Some(level)
                )
            } else {
                Err(Error::CustomError(
                    "ERROR: There are no articles of the specified type.",
                ))
            }
        } else {
            Err(Error::CustomError(
                    "ERROR: There are no articles of the specified type.",
            ))
        }
    }
}

pub fn get_info_item(id: u32, resources_path: &PathBuf) -> Result<(ItemInfo, ArticleType), Error> {
    let file_path = resources_path.join("items.json");
    let json_file = File::open(file_path).map_err(Error::IoError)?;
    let reader = BufReader::new(json_file);
    let items: Value = serde_json::from_reader(reader).unwrap();
    let items = items.as_object().unwrap();

    for (category, category_items) in items {
        match category_items
            .as_object()
            .unwrap()
            .keys()
            .find(|x| x.parse::<u32>().unwrap() == id)
        {
            Some(found) => {
                let mut info: ItemInfo =
                    serde_json::from_value(category_items[found].clone()).unwrap();
                if category == "chalice" {
                    info.extra_info = Some(json!({
                        "depth": &category_items[found]["depth"],
                        "area": &category_items[found]["area"],
                    }));
                }
                return Ok((info, ArticleType::from(category.as_str())));
            }
            None => (),
        }
    }
    Err(Error::CustomError(
        "ERROR: Failed to find info for the item.",
    ))
}

pub fn get_info_armor(id: u32, resources_path: &PathBuf) -> Result<(ItemInfo, ArticleType), Error> {
    let file_path = resources_path.join("armors.json");
    let json_file = File::open(file_path).map_err(Error::IoError)?;
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
            return Ok((info, ArticleType::Armor));
        }
        None => (),
    }
    Err(Error::CustomError(
        "ERROR: Failed to find info for the armor.",
    ))
}

pub fn get_info_weapon(
    mut id: u32,
    resources_path: &PathBuf,
) -> Result<(ItemInfo, ArticleType), Error> {
    let file_path = resources_path.join("weapons.json");
    let json_file = File::open(file_path).map_err(Error::IoError)?;
    let reader = BufReader::new(json_file);
    let weapons: Value = serde_json::from_reader(reader).unwrap();
    let weapons = weapons.as_object().unwrap();

    let weapon_mods = WeaponMods::try_from(id)?;
    if id != 12080000 && id != 6180000 {
        //Special case
        id = (id / 100000) * 100000; //Remove the weapon mods to be able to find its info
    }
    for (category, category_weapons) in weapons {
        match category_weapons
            .as_object()
            .unwrap()
            .keys()
            .find(|x| x.parse::<u32>().unwrap() == id)
        {
            Some(found) => {
                let mut info: ItemInfo =
                    serde_json::from_value(category_weapons[found].clone()).unwrap();
                let mut extra_info = json!({
                    "_base_damage": &category_weapons[found]["damage"],
                    "damage": &category_weapons[found]["damage"],
                    "upgrade_level": weapon_mods.upgrade_level,
                    "imprint": weapon_mods.imprint,
                });
                if weapon_mods.upgrade_level > 0 {
                    scale_weapon_info(&mut extra_info);
                }
                info.extra_info = Some(extra_info);
                return Ok((info, ArticleType::from(category.as_str())));
            }
            None => (),
        }
    }
    Err(Error::CustomError(
        "ERROR: Failed to find info for the weapon.",
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data_handling::{
        enums::SlotShape,
        slots::parse_equipped_gems,
        upgrades::parse_upgrades,
        utils::test_utils::{build_file_data, build_save_data, check_bytes},
    };

    #[test]
    fn inventory_edit_item() {
        let mut save = build_save_data("testsave0");
        assert!(check_bytes(
            &save.file,
            0x89cc,
            &[0x48, 0x80, 0xCF, 0xA8, 0x64, 0, 0, 0xB0, 0x64, 0, 0, 0x40, 0x01, 0, 0, 0]
        ));
        //Try to edit a key item
        let result = save
            .inventory
            .edit_item(&mut save.file, 0x00, 0xAAAAAAAA, 0xAABBCCDD, false);
        assert!(result.is_err());
        if let Err(error) = result {
            assert_eq!(
                error.to_string(),
                "Save error: ERROR: Key items cannot be edited."
            );
        }
        //Try wrong index
        let result = save
            .inventory
            .edit_item(&mut save.file, 0xAA, 0xAAAAAAAA, 0xAABBCCDD, false);
        assert!(result.is_err());
        if let Err(error) = result {
            assert_eq!(
                error.to_string(),
                "Save error: ERROR: The Article was not found in the inventory."
            );
        }

        save.inventory
            .edit_item(&mut save.file, 0x48, 0x64, 0xAABBCCDD, false)
            .unwrap();
        assert!(check_bytes(
            &save.file,
            0x89cc,
            &[0x48, 0x80, 0xCF, 0xA8, 0x64, 0, 0, 0xB0, 0x64, 0, 0, 0x40, 0xDD, 0xCC, 0xBB, 0xAA]
        ));
        assert_eq!(
            save.inventory
                .articles
                .get(&ArticleType::Consumable)
                .unwrap()[0]
                .amount,
            0xAABBCCDD
        );
    }

    #[test]
    fn test_parse_key_inventory() {
        let save = build_save_data("testsave0");
        let keys = save.inventory.articles.get(&ArticleType::Key).unwrap();
        assert_eq!(keys.len(), 7);

        //Item N0
        assert_eq!(keys[0].number, 107);
        assert_eq!(keys[0].id, u32::from_le_bytes([0xa9, 0x0f, 0x00, 0x00]));
        assert_eq!(
            keys[0].first_part,
            u32::from_le_bytes([0xa9, 0x0f, 0x00, 0xb0])
        );
        assert_eq!(
            keys[0].second_part,
            u32::from_le_bytes([0xa9, 0x0f, 0x00, 0x40])
        );
        assert_eq!(keys[0].amount, u32::from_le_bytes([0x01, 0x00, 0x00, 0x00]));
        assert_eq!(keys[0].article_type, ArticleType::Key);

        //Item N1
        assert_eq!(keys[1].number, 6);
        assert_eq!(keys[1].id, u32::from_le_bytes([0x12, 0x10, 0x00, 0x00]));
        assert_eq!(
            keys[1].first_part,
            u32::from_le_bytes([0x12, 0x10, 0x00, 0xb0])
        );
        assert_eq!(
            keys[1].second_part,
            u32::from_le_bytes([0x12, 0x10, 0x00, 0x40])
        );
        assert_eq!(keys[1].amount, u32::from_le_bytes([0x01, 0x00, 0x00, 0x00]));
        assert_eq!(keys[1].article_type, ArticleType::Key);

        //Item N2
        assert_eq!(keys[2].number, 0);
        assert_eq!(keys[2].id, u32::from_le_bytes([0xd8, 0x10, 0x00, 0x00]));
        assert_eq!(
            keys[2].first_part,
            u32::from_le_bytes([0xd8, 0x10, 0x00, 0xb0])
        );
        assert_eq!(
            keys[2].second_part,
            u32::from_le_bytes([0xd8, 0x10, 0x00, 0x40])
        );
        assert_eq!(keys[2].amount, u32::from_le_bytes([0x01, 0x00, 0x00, 0x00]));
        assert_eq!(keys[2].article_type, ArticleType::Key);

        //Item N3
        assert_eq!(keys[3].number, 1);
        assert_eq!(keys[3].id, u32::from_le_bytes([0x0e, 0x10, 0x00, 0x00]));
        assert_eq!(
            keys[3].first_part,
            u32::from_le_bytes([0x0e, 0x10, 0x00, 0xb0])
        );
        assert_eq!(
            keys[3].second_part,
            u32::from_le_bytes([0x0e, 0x10, 0x00, 0x40])
        );
        assert_eq!(keys[3].amount, u32::from_le_bytes([0x01, 0x00, 0x00, 0x00]));
        assert_eq!(keys[3].article_type, ArticleType::Key);

        //Item N4
        assert_eq!(keys[4].number, 2);
        assert_eq!(keys[4].id, u32::from_le_bytes([0xa0, 0x0f, 0x00, 0x00]));
        assert_eq!(
            keys[4].first_part,
            u32::from_le_bytes([0xa0, 0x0f, 0x00, 0xb0])
        );
        assert_eq!(
            keys[4].second_part,
            u32::from_le_bytes([0xa0, 0x0f, 0x00, 0x40])
        );
        assert_eq!(keys[4].amount, u32::from_le_bytes([0x01, 0x00, 0x00, 0x00]));
        assert_eq!(keys[4].article_type, ArticleType::Key);

        //Item N5
        assert_eq!(keys[5].number, 3);
        assert_eq!(keys[5].id, u32::from_le_bytes([0x07, 0x10, 0x00, 0x00]));
        assert_eq!(
            keys[5].first_part,
            u32::from_le_bytes([0x07, 0x10, 0x00, 0xb0])
        );
        assert_eq!(
            keys[5].second_part,
            u32::from_le_bytes([0x07, 0x10, 0x00, 0x40])
        );
        assert_eq!(keys[5].amount, u32::from_le_bytes([0x01, 0x00, 0x00, 0x00]));
        assert_eq!(keys[5].article_type, ArticleType::Key);

        //Item N6
        assert_eq!(keys[6].number, 4);
        assert_eq!(keys[6].id, u32::from_le_bytes([0xab, 0x0f, 0x00, 0x00]));
        assert_eq!(
            keys[6].first_part,
            u32::from_le_bytes([0xab, 0x0f, 0x00, 0xb0])
        );
        assert_eq!(
            keys[6].second_part,
            u32::from_le_bytes([0xab, 0x0f, 0x00, 0x40])
        );
        assert_eq!(keys[6].amount, u32::from_le_bytes([0x01, 0x00, 0x00, 0x00]));
        assert_eq!(keys[6].article_type, ArticleType::Key);
    }

    #[test]
    fn inventory_add_item() {
        let mut save = build_save_data("testsave0");
        assert_eq!(
            save.inventory
                .articles
                .get(&ArticleType::Consumable)
                .unwrap()
                .len(),
            17
        );
        assert!(check_bytes(
            &save.file,
            0x8ccc,
            &[
                0x78, 0xff, 0xff, 0xff, 0x00, 0x00, 0x00, 0x00, 0xff, 0xff, 0xff, 0xff, 0x00, 0x00,
                0x00, 0x00
            ]
        ));
        //Try to add an invalid item
        let result = save.inventory.add_item(&mut save.file, 0x00, 0x00, false);
        assert!(result.is_err());
        if let Err(error) = result {
            assert_eq!(
                error.to_string(),
                "Save error: ERROR: failed to find info for the item."
            );
        }

        //Add to the storage
        assert!(save
            .inventory
            .add_item(
                &mut save.file,
                u32::from_le_bytes([0x60, 0x04, 0x00, 0x00]),
                32,
                true
            )
            .is_ok());

        //Add to the inventory
        save.inventory
            .add_item(
                &mut save.file,
                u32::from_le_bytes([0x60, 0x04, 0x00, 0x00]),
                32,
                false,
            )
            .unwrap();
        assert_eq!(
            save.inventory
                .articles
                .get(&ArticleType::Consumable)
                .unwrap()
                .len(),
            19
        );
        assert!(check_bytes(
            &save.file,
            0x8ccc,
            &[
                0x78, 0xff, 0xff, 0xff, 0x60, 0x04, 0x00, 0xb0, 0x60, 0x04, 0x00, 0x40, 0x20, 0x00,
                0x00, 0x00, 0x79, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xff, 0xff, 0xff, 0xff,
                0x00, 0x00, 0x00, 0x00
            ]
        ));

        let mut upgrades = parse_upgrades(&save.file);
        let mut slots = parse_equipped_gems(&mut save.file, &mut upgrades);
        let inventory = Inventory::build(
            &save.file,
            save.file.offsets.inventory,
            save.file.offsets.key_inventory,
            &mut upgrades,
            &mut slots,
        );
        let consumables = inventory.articles.get(&ArticleType::Consumable).unwrap();
        let new_item = consumables.last().unwrap();
        assert_eq!(new_item.number, 120);
        assert_eq!(new_item.id, u32::from_le_bytes([0x60, 0x04, 0x00, 0x00]));
        assert_eq!(
            new_item.first_part,
            u32::from_le_bytes([0x60, 0x04, 0x00, 0xb0])
        );
        assert_eq!(
            new_item.second_part,
            u32::from_le_bytes([0x60, 0x04, 0x00, 0x40])
        );
        assert_eq!(
            new_item.amount,
            u32::from_le_bytes([0x20, 0x00, 0x00, 0x00])
        );
        assert_eq!(new_item.article_type, ArticleType::Consumable);

        //Add to a save without items in its storage
        let mut save = build_save_data("testsave7");
        save.storage
            .add_item(
                &mut save.file,
                u32::from_le_bytes([0x60, 0x04, 0x00, 0x00]),
                32,
                true,
            )
            .unwrap();
        let consumables = save.storage.articles.get(&ArticleType::Consumable).unwrap();
        let new_item = consumables.last().unwrap();
        assert_eq!(new_item.number, 1);
        assert_eq!(new_item.id, u32::from_le_bytes([0x60, 0x04, 0x00, 0x00]));
        assert_eq!(
            new_item.first_part,
            u32::from_le_bytes([0x60, 0x04, 0x00, 0xb0])
        );
        assert_eq!(
            new_item.second_part,
            u32::from_le_bytes([0x60, 0x04, 0x00, 0x40])
        );
        assert_eq!(
            new_item.amount,
            u32::from_le_bytes([0x20, 0x00, 0x00, 0x00])
        );
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
        assert_eq!(
            runes[0].source,
            u32::from_le_bytes([0x4a, 0x0d, 0x03, 0x80])
        );

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
        let weapons = save
            .inventory
            .articles
            .get(&ArticleType::RightHand)
            .unwrap();
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
        assert!(check_bytes(
            &save.file,
            0x8ccc,
            &[
                0x78, 0xff, 0xff, 0xff, 0x00, 0x00, 0x00, 0x00, 0xff, 0xff, 0xff, 0xff, 0x00, 0x00,
                0x00, 0x00
            ]
        ));

        //Add to the inventory
        save.inventory
            .add_upgrade(&mut save.file, rune.clone(), false);
        let runes = save.inventory.upgrades.get(&UpgradeType::Rune).unwrap();
        let mut rune2 = runes[1].clone();
        rune2.index = 0;
        assert_eq!(rune, rune2);
        assert_eq!(runes.len(), 2);
        assert!(check_bytes(
            &save.file,
            0x8ccc,
            &[
                0x78, 0xff, 0xff, 0xff, 0x42, 0x00, 0x80, 0xc0, 0xbf, 0x92, 0x01, 0x80, 0x01, 0x00,
                0x00, 0x00, 0x79, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xff, 0xff, 0xff, 0xff,
                0x00, 0x00, 0x00, 0x00
            ]
        ));

        //Add to a save without items in its storage
        let mut save = build_save_data("testsave7");
        save.storage.add_upgrade(&mut save.file, rune, true);
        let runes = save.storage.upgrades.get(&UpgradeType::Rune).unwrap();
        let new_rune = runes.last().unwrap();
        assert_eq!(new_rune.number, 1);
        assert_eq!(new_rune.id, u32::from_le_bytes([0x42, 0x00, 0x80, 0xc0]));
    }

    #[test]
    fn inventory_unequip_gem() {
        let mut save = build_save_data("testsave9");

        //Test error cases
        let result =
            save.inventory
                .unequip_gem(&mut save.file, ArticleType::Chalice, 500, 500, false);
        assert!(result.is_err());
        if let Err(error) = result {
            assert_eq!(
                error.to_string(),
                "Save error: ERROR: There are no articles of the specified type."
            );
        }

        let result =
            save.inventory
                .unequip_gem(&mut save.file, ArticleType::RightHand, 500, 500, false);
        assert!(result.is_err());
        if let Err(error) = result {
            assert_eq!(
                error.to_string(),
                "Save error: ERROR: article_index is invalid."
            );
        }

        let result =
            save.inventory
                .unequip_gem(&mut save.file, ArticleType::Consumable, 0, 500, false);
        assert!(result.is_err());
        if let Err(error) = result {
            assert_eq!(
                error.to_string(),
                "Save error: ERROR: The article has no slots."
            );
        }

        let result =
            save.inventory
                .unequip_gem(&mut save.file, ArticleType::RightHand, 0, 500, false);
        assert!(result.is_err());
        if let Err(error) = result {
            assert_eq!(
                error.to_string(),
                "Save error: ERROR: slot_index is invalid."
            );
        }

        let result =
            save.inventory
                .unequip_gem(&mut save.file, ArticleType::RightHand, 0, 4, false);
        assert!(result.is_err());
        if let Err(error) = result {
            assert_eq!(
                error.to_string(),
                "Save error: ERROR: The specified slot does not have a gem."
            );
        }

        let hunter_axe = save
            .inventory
            .articles
            .get_mut(&ArticleType::RightHand)
            .unwrap()
            .get_mut(0)
            .unwrap();
        let backup = hunter_axe.first_part;
        hunter_axe.first_part = 0;
        let result =
            save.inventory
                .unequip_gem(&mut save.file, ArticleType::RightHand, 0, 0, false);
        let hunter_axe = save
            .inventory
            .articles
            .get_mut(&ArticleType::RightHand)
            .unwrap()
            .get_mut(0)
            .unwrap();
        hunter_axe.first_part = backup;
        assert!(result.is_err());
        if let Err(error) = result {
            assert_eq!(
                error.to_string(),
                "Save error: ERROR: Failed to find the article in the file data."
            );
        }

        let hunter_axe = save
            .inventory
            .articles
            .get_mut(&ArticleType::RightHand)
            .unwrap()
            .get_mut(0)
            .unwrap();
        let gem = hunter_axe.slots.as_ref().unwrap()[0].clone().gem.unwrap();
        //Slots of the hunter axe
        assert!(check_bytes(
            &save.file,
            0x1570,
            &[
                0xd0, 0x01, 0x80, 0x80, 0x6c, 0x4c, 0x4c, 0x00, 0xfa, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x74, 0x00, 0x80, 0xc0,
                0x01, 0x00, 0x00, 0x00, 0x6f, 0x00, 0x80, 0xc0, 0x00, 0x00, 0x00, 0x80, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x80, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x80,
                0x00, 0x00, 0x00, 0x00, 0xd1, 0x01, 0x80, 0x80, 0x00, 0x12, 0x7a, 0x00, 0xfa, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x80,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x80, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x80, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x80, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x80, 0x00, 0x00, 0x00, 0x00, 0xd2, 0x01, 0x80, 0x90, 0x40, 0x19,
                0x01, 0x10,
            ]
        ));
        //Empty slot of the inventory
        assert!(check_bytes(
            &save.file,
            0x9158,
            &[
                0x68, 0x80, 0x93, 0xb9, 0x00, 0x00, 0x00, 0x00, 0xff, 0xff, 0xff, 0xff, 0x00, 0x00,
                0x00, 0x00
            ]
        ));

        save.inventory
            .unequip_gem(&mut save.file, ArticleType::RightHand, 0, 0, false)
            .unwrap();
        let mut upgrades = parse_upgrades(&save.file);
        let mut slots = parse_equipped_gems(&mut save.file, &mut upgrades);
        let mut inventory = Inventory::build(
            &save.file,
            save.file.offsets.inventory,
            save.file.offsets.key_inventory,
            &mut upgrades,
            &mut slots,
        );
        let new_gem = inventory
            .upgrades
            .get_mut(&UpgradeType::Gem)
            .unwrap()
            .get_mut(1)
            .unwrap();
        new_gem.number = gem.number;
        new_gem.index = gem.index;
        assert_eq!(*new_gem, gem);

        let hunter_axe = save
            .inventory
            .articles
            .get_mut(&ArticleType::RightHand)
            .unwrap()
            .get_mut(0)
            .unwrap();
        assert!(hunter_axe.slots.as_ref().unwrap()[0].clone().gem.is_none());
        assert!(check_bytes(
            &save.file,
            0x1570,
            &[
                0xd0, 0x01, 0x80, 0x80, 0x6c, 0x4c, 0x4c, 0x00, 0xfa, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x01, 0x00, 0x00, 0x00, 0x6f, 0x00, 0x80, 0xc0, 0x00, 0x00, 0x00, 0x80, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x80, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x80,
                0x00, 0x00, 0x00, 0x00, 0xd1, 0x01, 0x80, 0x80, 0x00, 0x12, 0x7a, 0x00, 0xfa, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x80,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x80, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x80, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x80, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x80, 0x00, 0x00, 0x00, 0x00, 0xd2, 0x01, 0x80, 0x90, 0x40, 0x19,
                0x01, 0x10,
            ]
        ));
        //The slot now has the gem
        assert!(check_bytes(
            &save.file,
            0x9158,
            &[
                0x68, 0x80, 0x93, 0xb9, 0x74, 0x00, 0x80, 0xc0, 0x62, 0xe4, 0x01, 0x80, 0x01, 0x00,
                0x00, 0x00
            ]
        ));
    }

    #[test]
    fn inventory_remove_upgrade() {
        let mut save = build_save_data("testsave9");
        //Test error cases
        let result = save
            .storage
            .remove_upgrade(&mut save.file, UpgradeType::Rune, 0, false);
        assert!(result.is_err());
        if let Err(error) = result {
            assert_eq!(
                error.to_string(),
                "Save error: ERROR: There are no upgrades of the specified type."
            );
        }

        let result = save
            .inventory
            .remove_upgrade(&mut save.file, UpgradeType::Rune, 999, false);
        assert!(result.is_err());
        if let Err(error) = result {
            assert_eq!(
                error.to_string(),
                "Save error: ERROR: upgrade_index is invalid."
            );
        }

        let gem = save
            .inventory
            .upgrades
            .get_mut(&UpgradeType::Gem)
            .unwrap()
            .get_mut(0)
            .unwrap();
        let backup = gem.id;
        gem.id = 0;
        let result = save
            .inventory
            .remove_upgrade(&mut save.file, UpgradeType::Gem, 0, false);
        let gem = save
            .inventory
            .upgrades
            .get_mut(&UpgradeType::Gem)
            .unwrap()
            .get_mut(0)
            .unwrap();
        gem.id = backup;
        let gem = gem.clone();
        assert!(result.is_err());
        if let Err(error) = result {
            assert_eq!(
                error.to_string(),
                "Save error: ERROR: Failed to find the upgrade in the inventory."
            );
        }

        //The inventory has two gems
        let gems = save.inventory.upgrades.get(&UpgradeType::Gem).unwrap();
        assert_eq!(gems.len(), 2);
        assert!(check_bytes(
            &save.file,
            0x8fe8,
            &[
                0x51, 0x40, 0x89, 0x13, 0x73, 0x00, 0x80, 0xc0, 0xf0, 0x49, 0x02, 0x80, 0x01, 0x00,
                0x00, 0x00
            ]
        ));

        //Run the method
        let removed_gem = save
            .inventory
            .remove_upgrade(&mut save.file, UpgradeType::Gem, 0, false)
            .unwrap();

        //Now the slot is empty and there is only one gem
        assert!(check_bytes(
            &save.file,
            0x8fe8,
            &[0x51, 0x40, 0x89, 0x13, 0, 0, 0, 0, 255, 255, 255, 255, 0, 0, 0, 0]
        ));
        let gems = save.inventory.upgrades.get(&UpgradeType::Gem).unwrap();
        assert_eq!(gems.len(), 1);
        assert_eq!(removed_gem, gem);

        //Rebuild the inventory to check the changes to file_data are valid
        let mut upgrades = parse_upgrades(&save.file);
        let mut slots = parse_equipped_gems(&mut save.file, &mut upgrades);
        let inventory = Inventory::build(
            &save.file,
            save.file.offsets.inventory,
            save.file.offsets.key_inventory,
            &mut upgrades,
            &mut slots,
        );
        let gems = inventory.upgrades.get(&UpgradeType::Gem).unwrap();
        assert_eq!(gems.len(), 1);
    }

    #[test]
    fn inventory_equip_gem() {
        let mut save = build_save_data("testsave9");

        //Test error cases
        let result =
            save.inventory
                .equip_gem(&mut save.file, 500, ArticleType::Chalice, 500, 500, false);
        assert!(result.is_err());
        if let Err(error) = result {
            assert_eq!(
                error.to_string(),
                "Save error: ERROR: There are no articles of the specified type."
            );
        }

        let result =
            save.inventory
                .equip_gem(&mut save.file, 500, ArticleType::RightHand, 500, 500, false);
        assert!(result.is_err());
        if let Err(error) = result {
            assert_eq!(
                error.to_string(),
                "Save error: ERROR: article_index is invalid."
            );
        }

        let result =
            save.inventory
                .equip_gem(&mut save.file, 500, ArticleType::Consumable, 0, 500, false);
        assert!(result.is_err());
        if let Err(error) = result {
            assert_eq!(
                error.to_string(),
                "Save error: ERROR: The article has no slots."
            );
        }

        let result =
            save.inventory
                .equip_gem(&mut save.file, 500, ArticleType::RightHand, 0, 500, false);
        assert!(result.is_err());
        if let Err(error) = result {
            assert_eq!(
                error.to_string(),
                "Save error: ERROR: slot_index is invalid."
            );
        }

        let result =
            save.inventory
                .equip_gem(&mut save.file, 500, ArticleType::RightHand, 0, 0, false);
        assert!(result.is_err());
        if let Err(error) = result {
            assert_eq!(
                error.to_string(),
                "Save error: ERROR: The specified slot already has a gem."
            );
        }

        let result =
            save.inventory
                .equip_gem(&mut save.file, 500, ArticleType::RightHand, 0, 4, false);
        assert!(result.is_err());
        if let Err(error) = result {
            assert_eq!(
                error.to_string(),
                "Save error: ERROR: upgrade_index is invalid."
            );
        }

        let hunter_axe = save
            .inventory
            .articles
            .get_mut(&ArticleType::RightHand)
            .unwrap()
            .get_mut(0)
            .unwrap();
        let backup = hunter_axe.first_part;
        hunter_axe.first_part = 0;
        let result =
            save.inventory
                .equip_gem(&mut save.file, 500, ArticleType::RightHand, 0, 4, false);
        let hunter_axe = save
            .inventory
            .articles
            .get_mut(&ArticleType::RightHand)
            .unwrap()
            .get_mut(0)
            .unwrap();
        hunter_axe.first_part = backup;
        assert!(result.is_err());
        if let Err(error) = result {
            assert_eq!(
                error.to_string(),
                "Save error: ERROR: Failed to find the article in the file data."
            );
        }

        //Get the gem to be equipped
        let gem = save.inventory.upgrades.get(&UpgradeType::Gem).unwrap()[0].clone();
        let hunter_axe = save
            .inventory
            .articles
            .get(&ArticleType::RightHand)
            .unwrap()
            .get(0)
            .unwrap();
        let slots = hunter_axe.slots.as_ref().unwrap();
        //The slot n4 is empty
        assert!(slots[4].gem.is_none());
        //The inventory has two gems
        assert_eq!(
            save.inventory
                .upgrades
                .get_mut(&UpgradeType::Gem)
                .unwrap()
                .len(),
            2
        );
        //Slots of the hunter axe
        assert!(check_bytes(
            &save.file,
            0x1570,
            &[
                0xd0, 0x01, 0x80, 0x80, 0x6c, 0x4c, 0x4c, 0x00, 0xfa, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x74, 0x00, 0x80, 0xc0,
                0x01, 0x00, 0x00, 0x00, 0x6f, 0x00, 0x80, 0xc0, 0x00, 0x00, 0x00, 0x80, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x80, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x80,
                0x00, 0x00, 0x00, 0x00,
            ]
        ));
        //Slot of the inventory with the gem
        assert!(check_bytes(
            &save.file,
            0x8fe8,
            &[
                0x51, 0x40, 0x89, 0x13, 0x73, 0x00, 0x80, 0xc0, 0xf0, 0x49, 0x02, 0x80, 0x01, 0x00,
                0x00, 0x00
            ]
        ));

        //Run the function
        save.inventory
            .equip_gem(&mut save.file, 0, ArticleType::RightHand, 0, 4, false)
            .unwrap();

        let hunter_axe = save
            .inventory
            .articles
            .get(&ArticleType::RightHand)
            .unwrap()
            .get(0)
            .unwrap();
        let slots = hunter_axe.slots.as_ref().unwrap();
        //Now the slot n4 contains the gem
        assert_eq!(*slots[4].gem.as_ref().unwrap(), gem);
        //And the inventory doesn't have that gem anymore
        assert_eq!(
            save.inventory
                .upgrades
                .get_mut(&UpgradeType::Gem)
                .unwrap()
                .len(),
            1
        );

        //Test again rebuilding the inventory
        let mut upgrades = parse_upgrades(&save.file);
        let mut slots = parse_equipped_gems(&mut save.file, &mut upgrades);
        let mut inventory = Inventory::build(
            &save.file,
            save.file.offsets.inventory,
            save.file.offsets.key_inventory,
            &mut upgrades,
            &mut slots,
        );
        assert_eq!(
            inventory.upgrades.get_mut(&UpgradeType::Gem).unwrap().len(),
            1
        );
        let hunter_axe = save
            .inventory
            .articles
            .get(&ArticleType::RightHand)
            .unwrap()
            .get(0)
            .unwrap();
        let slots = hunter_axe.slots.as_ref().unwrap();
        assert_eq!(*slots[4].gem.as_ref().unwrap(), gem);

        //Check the game is in the slots of the weapon
        assert!(check_bytes(
            &save.file,
            0x1570,
            &[
                0xd0, 0x01, 0x80, 0x80, 0x6c, 0x4c, 0x4c, 0x00, 0xfa, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x74, 0x00, 0x80, 0xc0,
                0x01, 0x00, 0x00, 0x00, 0x6f, 0x00, 0x80, 0xc0, 0x00, 0x00, 0x00, 0x80, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x80, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x80,
                0x73, 0x00, 0x80, 0xc0,
            ]
        ));

        //Now the slot in which the gem was is empty
        assert!(check_bytes(
            &save.file,
            0x8fe8,
            &[0x51, 0x40, 0x89, 0x13, 0, 0, 0, 0, 255, 255, 255, 255, 0, 0, 0, 0]
        ));
    }
}
