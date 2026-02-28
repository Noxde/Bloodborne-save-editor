// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{error::Error, fs::File, io::BufReader, sync::Mutex};
mod data_handling;

use data_handling::{
    appearance,
    article::Article,
    enums::{ArticleType, Location, SlotShape, UpgradeType},
    save::SaveData,
    upgrades::Upgrade,
};
use serde_json::{Value, json};
use tauri::{Manager, path::BaseDirectory};
struct MutexSave {
    data: Mutex<Option<SaveData>>,
}

fn main() -> Result<(), Box<dyn Error>> {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_shell::init())
        .manage(MutexSave {
            data: Mutex::new(None),
        })
        .invoke_handler(tauri::generate_handler![
            make_save,
            edit_quantity,
            save,
            return_weapons,
            return_armors,
            return_items,
            return_gem_effects,
            return_rune_effects,
            transform_item,
            edit_stat,
            edit_effect,
            edit_shape,
            equip_gem,
            unequip_gem,
            export_appearance,
            import_appearance,
            set_username,
            get_version,
            add_item,
            edit_slot,
            get_isz,
            fix_isz,
            get_playtime,
            set_playtime,
            set_flag,
            edit_coordinates,
            teleport,
            change_weapon_level
        ])
        .run(tauri::generate_context!())?;

    Ok(())
}

#[tauri::command]
fn set_flag(offset: usize, new_value: u8, state_save: tauri::State<MutexSave>) {
    let mut save_option = state_save.inner().data.lock().unwrap();
    let save = save_option.as_mut().unwrap();

    save.file.set_flag(offset, new_value);
}

#[tauri::command]
fn get_isz(state_save: tauri::State<MutexSave>) -> [u8; 2] {
    let mut save_option = state_save.inner().data.lock().unwrap();
    let save = save_option.as_mut().unwrap();

    return save.file.get_isz();
}

#[tauri::command]
fn fix_isz(state_save: tauri::State<MutexSave>) -> String {
    let mut save_option = state_save.inner().data.lock().unwrap();
    let save = save_option.as_mut().unwrap();

    save.file.fix_isz()
}

#[tauri::command]
fn get_playtime(state_save: tauri::State<MutexSave>) -> u32 {
    let mut save_option = state_save.inner().data.lock().unwrap();
    let save = save_option.as_mut().unwrap();

    save.file.get_playtime()
}

#[tauri::command]
fn set_playtime(new_playtime: [u8; 4], state_save: tauri::State<MutexSave>) {
    let mut save_option = state_save.inner().data.lock().unwrap();
    let save = save_option.as_mut().unwrap();

    save.file.set_playtime(new_playtime);
}

#[tauri::command]
fn make_save(
    path: &str,
    state_save: tauri::State<MutexSave>,
    handle: tauri::AppHandle,
) -> Result<Value, String> {
    let resource_path = handle
        .path().resolve("resources/", BaseDirectory::Resource).unwrap();

    match SaveData::build(path, resource_path) {
        Ok(s) => {
            let mut data = state_save.data.lock().unwrap();
            *data = Some(s.clone());
            Ok(serde_json::to_value(&s).map_err(|x| x.to_string())?)
        }
        Err(_) => Err("Failed to load file, make sure its a decrypted character.".to_string()),
    }
}

#[tauri::command]
fn edit_quantity(
    number: u8,
    id: u32,
    value: u32,
    is_storage: bool,
    state_save: tauri::State<MutexSave>,
) -> Result<Value, String> {
    let mut save_option = state_save.inner().data.lock().unwrap();
    let save = save_option.as_mut().unwrap();

    if !is_storage {
        match save
            .inventory
            .edit_item(&mut save.file, number, id, value, is_storage)
        {
            Ok(_) => Ok(serde_json::to_value(&save).map_err(|x| x.to_string())?),
            Err(e) => Err(e.to_string()),
        }
    } else {
        match save
            .storage
            .edit_item(&mut save.file, number, id, value, is_storage)
        {
            Ok(_) => Ok(serde_json::to_value(&save).map_err(|x| x.to_string())?),
            Err(e) => Err(e.to_string()),
        }
    }
}

#[tauri::command]
fn save(path: String, state_save: tauri::State<MutexSave>) -> Result<&str, &str> {
    let mut save_option = state_save.inner().data.lock().unwrap();
    let save = save_option.as_mut().unwrap();

    match save.file.save(&path) {
        Ok(_) => Ok("Changes saved."),
        Err(_) => Err("Failed to save changes."),
    }
}

#[tauri::command]
fn return_weapons(state_save: tauri::State<MutexSave>) -> Value {
    let save_option = state_save.inner().data.lock().unwrap();
    let save = save_option.as_ref().unwrap();
    let file_path = save.file.resources_path.join("weapons.json");
    let json_file = File::open(file_path).unwrap();
    let reader = BufReader::new(json_file);
    let weapons: Value = serde_json::from_reader(reader).unwrap();

    weapons
}

#[tauri::command]
fn return_armors(state_save: tauri::State<MutexSave>) -> Value {
    let save_option = state_save.inner().data.lock().unwrap();
    let save = save_option.as_ref().unwrap();
    let file_path = save.file.resources_path.join("armors.json");
    let json_file = File::open(file_path).unwrap();
    let reader = BufReader::new(json_file);
    let armors: Value = serde_json::from_reader(reader).unwrap();

    armors
}

#[tauri::command]
fn return_items(state_save: tauri::State<MutexSave>) -> Value {
    let save_option = state_save.inner().data.lock().unwrap();
    let save = save_option.as_ref().unwrap();
    let file_path = save.file.resources_path.join("items.json");
    let json_file = File::open(file_path).unwrap();
    let reader = BufReader::new(json_file);
    let items: Value = serde_json::from_reader(reader).unwrap();

    items
}

#[tauri::command]
fn return_gem_effects(state_save: tauri::State<MutexSave>) -> Value {
    let save_option = state_save.inner().data.lock().unwrap();
    let save = save_option.as_ref().unwrap();
    let file_path = save.file.resources_path.join("upgrades.json");
    let json_file = File::open(file_path).unwrap();
    let reader = BufReader::new(json_file);
    let upgrade_json: Value = serde_json::from_reader(reader).unwrap();

    upgrade_json["gemEffects"].clone()
}

#[tauri::command]
fn return_rune_effects(state_save: tauri::State<MutexSave>) -> Value {
    let save_option = state_save.inner().data.lock().unwrap();
    let save = save_option.as_ref().unwrap();
    let file_path = save.file.resources_path.join("upgrades.json");
    let json_file = File::open(file_path).unwrap();
    let reader = BufReader::new(json_file);
    let upgrade_json: Value = serde_json::from_reader(reader).unwrap();

    upgrade_json["runeEffects"].clone()
}

#[tauri::command]
fn transform_item(
    index: usize,
    id: u32,
    new_id: u32,
    article_type: ArticleType,
    is_storage: bool,
    state_save: tauri::State<MutexSave>,
) -> Result<Value, String> {
    let mut save_option = state_save.inner().data.lock().unwrap();
    let save = save_option.as_mut().unwrap();

    let category = {
        if !is_storage {
            save.inventory.articles.get_mut(&article_type).unwrap()
        } else {
            save.storage.articles.get_mut(&article_type).unwrap()
        }
    };
    let item = category
        .iter_mut()
        .find(|x| x.id == id && x.index == index)
        .unwrap();

    let old_type = item.article_type;

    match item.transform(&mut save.file, new_id, is_storage) {
        Ok(_) => {
            // Check if the article type has changed
            if item.article_type != old_type {
                let moved_item = item.clone();

                // Remove the item from the old category
                if let Some(old_category) = save.inventory.articles.get_mut(&old_type) {
                    old_category.retain(|x| x.index != index);
                }

                // Find or create the new category using item.article_type
                let new_category = save
                    .inventory
                    .articles
                    .entry(moved_item.article_type)
                    .or_insert_with(Vec::new);

                // Add the item to the new category
                new_category.push(moved_item);
            }

            Ok(serde_json::to_value(&save).map_err(|x| x.to_string())?)
        }
        Err(e) => Err(e.to_string()),
    }
}

#[tauri::command]
fn edit_stat(
    rel_offset: isize,
    length: usize,
    times: usize,
    value: u32,
    state_save: tauri::State<MutexSave>,
) {
    let mut save_option = state_save.inner().data.lock().unwrap();
    let save = save_option.as_mut().unwrap();

    save.file.edit(rel_offset, length, times, value);
}

#[tauri::command]
fn edit_effect(
    new_effect_id: u32,
    index: usize,
    info: Value,
    state_save: tauri::State<MutexSave>,
) -> Result<Value, String> {
    let mut save_option = state_save.inner().data.lock().unwrap();
    let save: &mut SaveData = save_option.as_mut().unwrap();
    let upgrade: Option<*mut Upgrade>;

    let location: Location = {
        let is_storage: bool = serde_json::from_value(info["isStorage"].clone()).unwrap();

        if is_storage {
            Location::Storage
        } else {
            Location::Inventory
        }
    };

    if let Some(equipped) = info.get("equipped") {
        let article_type: ArticleType =
            serde_json::from_value(equipped["articleType"].clone()).unwrap();
        let article_index: usize =
            serde_json::from_value(equipped["articleIndex"].clone()).unwrap();
        let slot_index: usize = serde_json::from_value(equipped["slotIndex"].clone()).unwrap();

        upgrade = save
            .get_equipped_upgrade_mut(location, article_type, article_index, slot_index)
            .map(|u| u as *mut _);
    } else {
        let upgrade_type: UpgradeType =
            serde_json::from_value(info["upgradeType"].clone()).unwrap();
        let upgrade_index: usize = serde_json::from_value(info["upgradeIndex"].clone()).unwrap();

        upgrade = save
            .get_upgrade_mut(location, upgrade_type, upgrade_index)
            .map(|u| u as *mut _);
    }

    unsafe {
        match (*upgrade.unwrap()).change_effect(&mut save.file, new_effect_id, index) {
            Ok(_) => Ok(serde_json::to_value(&save).map_err(|x| x.to_string())?),
            Err(_) => Err("Failed to edit the upgrade's effect".to_string()),
        }
    }
}

#[tauri::command]
fn edit_shape(
    new_shape: String,
    info: Value,
    state_save: tauri::State<MutexSave>,
) -> Result<Value, String> {
    let mut save_option = state_save.inner().data.lock().unwrap();
    let save: &mut SaveData = save_option.as_mut().unwrap();
    let upgrade: Option<*mut Upgrade>;

    let location: Location = {
        let is_storage: bool = serde_json::from_value(info["isStorage"].clone()).unwrap();

        if is_storage {
            Location::Storage
        } else {
            Location::Inventory
        }
    };

    if let Some(equipped) = info.get("equipped") {
        let article_type: ArticleType =
            serde_json::from_value(equipped["articleType"].clone()).unwrap();
        let article_index: usize =
            serde_json::from_value(equipped["articleIndex"].clone()).unwrap();
        let slot_index: usize = serde_json::from_value(equipped["slotIndex"].clone()).unwrap();

        upgrade = save
            .get_equipped_upgrade_mut(location, article_type, article_index, slot_index)
            .map(|u| u as *mut _);
    } else {
        let upgrade_type: UpgradeType =
            serde_json::from_value(info["upgradeType"].clone()).unwrap();
        let upgrade_index: usize = serde_json::from_value(info["upgradeIndex"].clone()).unwrap();

        upgrade = save
            .get_upgrade_mut(location, upgrade_type, upgrade_index)
            .map(|u| u as *mut _);
    }

    unsafe {
        match (*upgrade.unwrap()).change_shape(&mut save.file, new_shape) {
            Ok(_) => Ok(serde_json::to_value(&save).map_err(|x| x.to_string())?),
            Err(_) => Err("Failed to edit the upgrade's shape".to_string()),
        }
    }
}

#[tauri::command]
fn edit_slot(
    is_storage: bool,
    article_type: ArticleType,
    article_index: usize,
    slot_index: usize,
    new_shape: SlotShape,
    state_save: tauri::State<MutexSave>,
) -> Result<Value, String> {
    let mut save_option = state_save.inner().data.lock().unwrap();
    let save: &mut SaveData = save_option.as_mut().unwrap();

    let location = if is_storage {
        Location::Storage
    } else {
        Location::Inventory
    };
    let article: Option<*mut Article>;

    article = save
        .get_article_mut(location, article_type, article_index)
        .map(|u| u as *mut _);

    unsafe {
        match (*article.unwrap()).change_slot_shape(&mut save.file, slot_index, new_shape) {
            Ok(_) => Ok(serde_json::to_value(&save).map_err(|x| x.to_string())?),
            Err(e) => Err(e.to_string()),
        }
    }
}

#[tauri::command]
fn equip_gem(
    upgrade_index: usize,
    article_type: ArticleType,
    article_index: usize,
    slot_index: usize,
    is_storage: bool,
    state_save: tauri::State<MutexSave>,
) -> Result<Value, String> {
    let mut save_option = state_save.inner().data.lock().unwrap();
    let save = save_option.as_mut().unwrap();

    let result = if is_storage {
        save.storage.equip_gem(
            &mut save.file,
            upgrade_index,
            article_type,
            article_index,
            slot_index,
            is_storage,
        )
    } else {
        save.inventory.equip_gem(
            &mut save.file,
            upgrade_index,
            article_type,
            article_index,
            slot_index,
            is_storage,
        )
    };

    match result {
        Ok(_) => Ok(serde_json::to_value(&save).map_err(|x| x.to_string())?),
        Err(e) => Err(e.to_string()),
    }
}

#[tauri::command]
fn unequip_gem(
    article_type: ArticleType,
    article_index: usize,
    slot_index: usize,
    is_storage: bool,
    state_save: tauri::State<MutexSave>,
) -> Result<Value, String> {
    let mut save_option = state_save.inner().data.lock().unwrap();
    let save = save_option.as_mut().unwrap();

    let result = if is_storage {
        save.storage.unequip_gem(
            &mut save.file,
            article_type,
            article_index,
            slot_index,
            is_storage,
        )
    } else {
        save.inventory.unequip_gem(
            &mut save.file,
            article_type,
            article_index,
            slot_index,
            is_storage,
        )
    };

    match result {
        Ok(_) => Ok(serde_json::to_value(&save).map_err(|x| x.to_string())?),
        Err(e) => Err(e.to_string()),
    }
}

#[tauri::command]
fn export_appearance(path: &str, state_save: tauri::State<MutexSave>) -> Result<String, String> {
    let mut save_option = state_save.inner().data.lock().unwrap();
    let save = save_option.as_mut().unwrap();

    match appearance::export(&save.file, path) {
        Ok(_) => Ok("Successfully exported".to_string()),
        Err(_) => Err("There was an error exporting the face".to_string()),
    }
}

#[tauri::command]
fn import_appearance(path: &str, state_save: tauri::State<MutexSave>) -> Result<String, String> {
    let mut save_option = state_save.inner().data.lock().unwrap();
    let save: &mut SaveData = save_option.as_mut().unwrap();

    match appearance::import(&mut save.file, path) {
        Ok(_) => Ok("Successfully imported".to_string()),
        Err(_) => Err("The imported file is not a face".to_string()),
    }
}

#[tauri::command]
fn set_username(
    new_username: String,
    state_save: tauri::State<MutexSave>,
) -> Result<String, String> {
    let mut save_option = state_save.inner().data.lock().unwrap();
    let save = save_option.as_mut().unwrap();

    match save.username.set(&mut save.file, new_username) {
        Ok(_) => Ok("Successfully changed name".to_string()),
        Err(_) => Err("Failed to change name".to_string()),
    }
}

#[tauri::command]
fn get_version() -> String {
    env!("CARGO_PKG_VERSION").to_string()
}

#[tauri::command]
fn add_item(
    id: u32,
    quantity: u32,
    is_storage: bool,
    state_save: tauri::State<MutexSave>,
) -> Result<Value, String> {
    let mut save_option = state_save.inner().data.lock().unwrap();
    let save = save_option.as_mut().unwrap();

    if !is_storage {
        match save
            .inventory
            .add_item(&mut save.file, id, quantity, is_storage)
        {
            Ok(_) => Ok(serde_json::to_value(&save).map_err(|x| x.to_string())?),
            Err(_) => Err("Failed to add the item".to_string()),
        }
    } else {
        match save
            .storage
            .add_item(&mut save.file, id, quantity, is_storage)
        {
            Ok(_) => Ok(serde_json::to_value(&save).map_err(|x| x.to_string())?),
            Err(_) => Err("Failed to add the item".to_string()),
        }
    }
}

#[tauri::command]
fn edit_coordinates(x: f32, y: f32, z: f32, state_save: tauri::State<MutexSave>) {
    let mut save_option = state_save.inner().data.lock().unwrap();
    let save = save_option.as_mut().unwrap();

    save.position.coordinates.edit(&mut save.file, x, y, z);
}

#[tauri::command]
fn teleport(x: f32, y: f32, z: f32, map_id: Vec<u8>, state_save: tauri::State<MutexSave>) {
    let mut save_option = state_save.inner().data.lock().unwrap();
    let save: &mut SaveData = save_option.as_mut().unwrap();
    let le_map = [00, 00, map_id[1], map_id[0]];

    for (i, j) in (0x04..0x08).enumerate() {
            save.file.bytes[j] = le_map[i];
    }

    save.position.coordinates.edit(&mut save.file, x, y, z);
}

#[tauri::command]
fn change_weapon_level(
    article_type: ArticleType,
    article_index: usize,
    slot_index: usize,
    is_storage: bool,
    level: u8,
    state_save: tauri::State<MutexSave>
) -> Result<Value, String> {
    let mut save_option = state_save.inner().data.lock().unwrap();
    let save: &mut SaveData = save_option.as_mut().unwrap();

    let result = if is_storage {
        save.storage.change_weapon_level(
            &mut save.file,
            article_type,
            article_index,
            slot_index,
            is_storage,
            level
        )
    } else {
        save.inventory.change_weapon_level(
            &mut save.file,
            article_type,
            article_index,
            slot_index,
            is_storage,
            level
        )
    };

    match result {
        Ok(weapon) => Ok(json!({
            "save": serde_json::to_value(&save).map_err(|x| x.to_string())?,
            "weapon": weapon
        })),
        Err(e) => Err(e.to_string()),
    }
}