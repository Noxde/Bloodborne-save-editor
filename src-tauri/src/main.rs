// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{error::Error, fs::File, io::BufReader, sync::Mutex};
mod data_handling;

use data_handling::{enums::{ArticleType, UpgradeType}, save::SaveData, appearance};
use serde_json::Value;
struct MutexSave {
    data: Mutex<Option<SaveData>>
}

fn main() -> Result<(), Box<dyn Error>> {

    tauri::Builder::default().manage(MutexSave { data: Mutex::new(None) })
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
            export_appearance,
            import_appearance,
            set_username,
            get_version
        ])
        .run(tauri::generate_context!())?;

    Ok(())
}

#[tauri::command]
fn make_save(path: &str, state_save: tauri::State<MutexSave>, handle: tauri::AppHandle) -> Result<Value, String> {
   let resource_path = handle.path_resolver()
      .resolve_resource("resources")
      .expect("failed to resolve resource");

    match SaveData::build(path, resource_path) {
        Ok(s) => {
            let mut data = state_save.data.lock().unwrap();
            *data = Some(s.clone());
            Ok(serde_json::json!({
                "username": &s.username,
                "inventory": &s.inventory,
                "storage": &s.storage,
                "upgrades": &s.upgrades,
                "stats": &s.stats
            }))
        },
        Err(e) => {
            Err("Failed to load file, make sure its a decrypted character.".to_string())
        }
    }
}

#[tauri::command]
fn edit_quantity(index: u8, id: u32, value: u32, is_storage: bool, state_save: tauri::State<MutexSave>) -> Result<Value, String> {
    let mut save_option = state_save.inner().data.lock().unwrap();
    let save = save_option.as_mut().unwrap();

    if !is_storage {
        match save.inventory.edit_item(&mut save.file, index, id, value, is_storage) {
            Ok(_) => Ok(serde_json::json!({
                "username": &save.username,
                "inventory": &save.inventory,
                "storage": &save.storage,
                "upgrades": &save.upgrades,
                "stats": &save.stats
            })),
            Err(e) => Err(e.to_string())
        }
    } else {
        match save.storage.edit_item(&mut save.file, index, id, value, is_storage) {
            Ok(_) => Ok(serde_json::json!({
                "username": &save.username,
                "inventory": &save.inventory,
                "storage": &save.storage,
                "upgrades": &save.upgrades,
                "stats": &save.stats
            })),
            Err(e) => Err(e.to_string())
        }
    }
}

#[tauri::command]
fn save(path: String, state_save: tauri::State<MutexSave>) -> Result<&str, &str> {
    let mut save_option = state_save.inner().data.lock().unwrap();
    let save = save_option.as_mut().unwrap();

    match save.file.save(&path) {
        Ok(_) => Ok("Changes saved."),
        Err(_) => Err("Failed to save changes.") 
    }
}

#[tauri::command]
fn return_weapons(state_save: tauri::State<MutexSave>) -> Value {
    let save_option = state_save.inner().data.lock().unwrap();
    let save = save_option.as_ref().unwrap();
    let file_path = save.file.resources_path.join("weapons.json");
    let json_file =  File::open(file_path).unwrap();
    let reader = BufReader::new(json_file);
    let weapons: Value = serde_json::from_reader(reader).unwrap();

    weapons
}

#[tauri::command]
fn return_armors(state_save: tauri::State<MutexSave>) -> Value {
    let save_option = state_save.inner().data.lock().unwrap();
    let save = save_option.as_ref().unwrap();
    let file_path = save.file.resources_path.join("armors.json");
    let json_file =  File::open(file_path).unwrap();
    let reader = BufReader::new(json_file);
    let armors: Value = serde_json::from_reader(reader).unwrap();

    armors
}

#[tauri::command]
fn return_items(state_save: tauri::State<MutexSave>) -> Value {
    let save_option = state_save.inner().data.lock().unwrap();
    let save = save_option.as_ref().unwrap();
    let file_path = save.file.resources_path.join("items.json");
    let json_file =  File::open(file_path).unwrap();
    let reader = BufReader::new(json_file);
    let items: Value = serde_json::from_reader(reader).unwrap();

    items
}

#[tauri::command]
fn return_gem_effects(state_save: tauri::State<MutexSave>) -> Value {
    let save_option = state_save.inner().data.lock().unwrap();
    let save = save_option.as_ref().unwrap();
    let file_path = save.file.resources_path.join("upgrades.json");
    let json_file =  File::open(file_path).unwrap();
    let reader = BufReader::new(json_file);
    let upgrade_json: Value = serde_json::from_reader(reader).unwrap();

    upgrade_json["gemEffects"].clone()
}

#[tauri::command]
fn return_rune_effects(state_save: tauri::State<MutexSave>) -> Value {
    let save_option = state_save.inner().data.lock().unwrap();
    let save = save_option.as_ref().unwrap();
    let file_path = save.file.resources_path.join("upgrades.json");
    let json_file =  File::open(file_path).unwrap();
    let reader = BufReader::new(json_file);
    let upgrade_json: Value = serde_json::from_reader(reader).unwrap();

    upgrade_json["runeEffects"].clone()
}


#[tauri::command]
fn transform_item(index: u8, id: u32, new_id: u32, article_type: ArticleType, is_storage: bool, state_save: tauri::State<MutexSave>) -> Result<Value, String> {
    let mut save_option = state_save.inner().data.lock().unwrap();
    let save = save_option.as_mut().unwrap();

    let category = {
        if !is_storage {
            save.inventory.articles.get_mut(&article_type).unwrap()
        } else {
            save.storage.articles.get_mut(&article_type).unwrap()
        }
    };
    let item = category.iter_mut().find(|x| x.id == id && x.index == index).unwrap();
    
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
                let new_category = save.inventory.articles.entry(moved_item.article_type).or_insert_with(Vec::new);

                // Add the item to the new category
                new_category.push(moved_item);
            }

            Ok(serde_json::json!({
                "username": &save.username,
                "inventory": &save.inventory,
                "storage": &save.storage,
                "upgrades": &save.upgrades,
                "stats": &save.stats
            }))
        },
        Err(e) => Err(e.to_string())
    }
}

#[tauri::command]
fn edit_stat(rel_offset: isize, length: usize, times: usize, value: u32, state_save: tauri::State<MutexSave>) {
    let mut save_option = state_save.inner().data.lock().unwrap();
    let save = save_option.as_mut().unwrap();

    save.file.edit(rel_offset, length, times, value);
}

#[tauri::command]
fn edit_effect(upgrade_id: u32, upgrade_type: UpgradeType, new_effect_id: u32, index: usize, state_save: tauri::State<MutexSave>) -> Result<Value, String> {
    let mut save_option = state_save.inner().data.lock().unwrap();
    let save = save_option.as_mut().unwrap();

    let upgrades_vec = {
      match upgrade_type {
        UpgradeType::Gem => save.upgrades.get_mut(&UpgradeType::Gem).unwrap(),
        UpgradeType::Rune => save.upgrades.get_mut(&UpgradeType::Rune).unwrap()
      }
    };

    let upgrade = upgrades_vec.iter_mut().find(|x| x.id == upgrade_id).unwrap();

    match upgrade.change_effect(&mut save.file, new_effect_id, index) {
        Ok(_) => Ok(serde_json::json!({
            "username": &save.username,
            "inventory": &save.inventory,
            "storage": &save.storage,
            "upgrades": &save.upgrades,
            "stats": &save.stats
        })),
        Err(_) => Err("Failed to edit the upgrade's effect".to_string())
    }
}

#[tauri::command]
fn edit_shape(upgrade_id: u32, upgrade_type: UpgradeType, new_shape: String, state_save: tauri::State<MutexSave>) -> Result<Value, String> {
    let mut save_option = state_save.inner().data.lock().unwrap();
    let save = save_option.as_mut().unwrap();
    
    let upgrades_vec = {
        match upgrade_type {
          UpgradeType::Gem => save.upgrades.get_mut(&UpgradeType::Gem).unwrap(),
          UpgradeType::Rune => save.upgrades.get_mut(&UpgradeType::Rune).unwrap()
        }
      };
  
      let upgrade = upgrades_vec.iter_mut().find(|x| x.id == upgrade_id).unwrap();
  
    match upgrade.change_shape(&mut save.file, new_shape) {
        Ok(_) => Ok(serde_json::json!({
            "username": &save.username,
            "inventory": &save.inventory,
            "storage": &save.storage,
            "upgrades": &save.upgrades,
            "stats": &save.stats
        })),
        Err(_) => Err("Failed to edit the upgrade's effect".to_string())
    }
}

#[tauri::command]
fn export_appearance(path: &str, state_save: tauri::State<MutexSave>) -> Result<String, String> {
    let mut save_option = state_save.inner().data.lock().unwrap();
    let save = save_option.as_mut().unwrap();
    
    match appearance::export(&save.file, path) {
        Ok(_) => Ok("Successfully exported".to_string()),
        Err(_) => Err("There was an error exporting the face".to_string())
    }
}

#[tauri::command]
fn import_appearance(path: &str, state_save: tauri::State<MutexSave>) -> Result<String, String> {
    let mut save_option = state_save.inner().data.lock().unwrap();
    let save = save_option.as_mut().unwrap();
    
    match appearance::import(&mut save.file, path) {
        Ok(_) => Ok("Successfully imported".to_string()),
        Err(_) => Err("The imported file is not a face".to_string())
    }
}

#[tauri::command]
fn set_username(new_username: String, state_save: tauri::State<MutexSave>) -> Result<String, String> {
    let mut save_option = state_save.inner().data.lock().unwrap();
    let save = save_option.as_mut().unwrap();
    
    match save.username.set(&mut save.file, new_username) {
        Ok(_) => Ok("Successfully changed name".to_string()),
        Err(_) => Err("Failed to change name".to_string())
    }
}

#[tauri::command]
fn get_version() -> String {
    env!("CARGO_PKG_VERSION").to_string()
}