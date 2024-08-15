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
            transform_item
            ])
        .run(tauri::generate_context!())?;

    Ok(())
}

#[tauri::command]
fn make_save(path: &str, state_save: tauri::State<MutexSave>) -> Result<SaveData, String> {
    if let Ok(s) = SaveData::build(path) {
        let mut data = state_save.data.lock().unwrap();
        *data = Some(s.clone());
        Ok(s)
    } else {
        Err("Failed to load save, make sure its a decrypted character.".to_string())
    }
}

// Lags a bit
#[tauri::command]
fn edit_quantity(index: u8, value: u32, state_save: tauri::State<MutexSave>) -> Result<SaveData, ()> {
    let mut save_option = state_save.inner().data.lock().unwrap();
    let save = save_option.as_mut().unwrap();

    match save.inventory.edit_item(&mut save.file, index, value) {
        Ok(_) => Ok(save.clone()),
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
fn return_weapons() -> Value {
    let json_file = File::open("weapons.json").unwrap();
    let reader = BufReader::new(json_file);
    let weapons: Value = serde_json::from_reader(reader).unwrap();

    weapons
}

#[tauri::command]
fn return_armors() -> Value {
    let json_file = File::open("armors.json").unwrap();
    let reader = BufReader::new(json_file);
    let armors: Value = serde_json::from_reader(reader).unwrap();

    armors
}

#[tauri::command]
fn return_items() -> Value {
    let json_file = File::open("items.json").unwrap();
    let reader = BufReader::new(json_file);
    let items: Value = serde_json::from_reader(reader).unwrap();

    items
}

#[tauri::command]
fn transform_item(id: u32, new_id: u32, article_type: ArticleType, state_save: tauri::State<MutexSave>) -> Result<SaveData, &str> {
    let mut save_option = state_save.inner().data.lock().unwrap();
    let save = save_option.as_mut().unwrap();

    let category = save.inventory.articles.get_mut(&article_type).unwrap();
    let item = category.iter_mut().find(|x| x.id == id).unwrap();
    match item.transform(&mut save.file, new_id) {
        Ok(_) => Ok(save.clone()),
        Err(_) => Err("Failed to transform item")
    }
}