// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{error::Error, fs::File, io::BufReader, sync::Mutex};
mod data_handling;

use data_handling::{enums::ArticleType, save::SaveData};
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
            transform_item,
            edit_stat,
            ])
        .run(tauri::generate_context!())?;

    Ok(())
}

#[tauri::command]
fn make_save(path: &str, state_save: tauri::State<MutexSave>, handle: tauri::AppHandle) -> Result<Value, String> {
   let resource_path = handle.path_resolver()
      .resolve_resource("resources")
      .expect("failed to resolve resource");
   let resource_path = resource_path.to_str().unwrap();

    if let Ok(s) = SaveData::build(path, resource_path) {
        let mut data = state_save.data.lock().unwrap();
        *data = Some(s.clone());
        Ok(serde_json::json!({
            "inventory": &s.inventory,
            "stats": &s.stats
        }))
    } else {
        Err("Failed to load save, make sure its a decrypted character.".to_string())
    }
}

#[tauri::command]
fn edit_quantity(index: u8, id: u32, value: u32, state_save: tauri::State<MutexSave>) -> Result<Value, ()> {
    let mut save_option = state_save.inner().data.lock().unwrap();
    let save = save_option.as_mut().unwrap();

    match save.inventory.edit_item(&mut save.file, index, id, value) {
        Ok(_) => Ok(serde_json::json!({
            "inventory": &save.inventory,
            "stats": &save.stats
        })),
        Err(_) => Err(())
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
    let resources_path = save.file.resources_path.as_str();
    let json_file =  File::open(format!("{resources_path}/weapons.json")).unwrap();
    let reader = BufReader::new(json_file);
    let weapons: Value = serde_json::from_reader(reader).unwrap();

    weapons
}

#[tauri::command]
fn return_armors(state_save: tauri::State<MutexSave>) -> Value {
    let save_option = state_save.inner().data.lock().unwrap();
    let save = save_option.as_ref().unwrap();
    let resources_path = save.file.resources_path.as_str();
    let json_file =  File::open(format!("{resources_path}/armors.json")).unwrap();
    let reader = BufReader::new(json_file);
    let armors: Value = serde_json::from_reader(reader).unwrap();

    armors
}

#[tauri::command]
fn return_items(state_save: tauri::State<MutexSave>) -> Value {
    let save_option = state_save.inner().data.lock().unwrap();
    let save = save_option.as_ref().unwrap();
    let resources_path = save.file.resources_path.as_str();
    let json_file =  File::open(format!("{resources_path}/items.json")).unwrap();
    let reader = BufReader::new(json_file);
    let items: Value = serde_json::from_reader(reader).unwrap();

    items
}

#[tauri::command]
fn transform_item(id: u32, new_id: u32, article_type: ArticleType, state_save: tauri::State<MutexSave>) -> Result<Value, &str> {
    let mut save_option = state_save.inner().data.lock().unwrap();
    let save = save_option.as_mut().unwrap();

    let category = save.inventory.articles.get_mut(&article_type).unwrap();
    let item = category.iter_mut().find(|x| x.id == id).unwrap();
    match item.transform(&mut save.file, new_id) {
        Ok(_) => Ok(serde_json::json!({
            "inventory": &save.inventory,
            "stats": &save.stats
        })),
        Err(_) => Err("Failed to transform item")
    }
}

#[tauri::command]
fn edit_stat(rel_offset: isize, length: usize, times: usize, value: u32, state_save: tauri::State<MutexSave>) {
    let mut save_option = state_save.inner().data.lock().unwrap();
    let save = save_option.as_mut().unwrap();

    save.file.edit(rel_offset, length, times, value);
}
