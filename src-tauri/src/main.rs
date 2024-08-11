// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{error::Error, sync::Mutex};
mod data_handling;

use data_handling::save::SaveData;
struct MutexSave {
    data: Mutex<Option<SaveData>>
}

fn main() -> Result<(), Box<dyn Error>> {

    tauri::Builder::default().manage(MutexSave { data: Mutex::new(None) })
        .invoke_handler(tauri::generate_handler![
            make_save,
            edit_quantity,
            save
            ])
        .run(tauri::generate_context!())?;

    Ok(())
}

#[tauri::command]
fn make_save(path: &str, state_save: tauri::State<MutexSave>) -> Option<SaveData> {
    if let Ok(s) = SaveData::build(path) {
        let mut data = state_save.data.lock().unwrap();
        *data = Some(s.clone());
        Some(s)
    } else {
        None
    }
}

// Lags a bit
#[tauri::command]
fn edit_quantity(index: u8, value: u32, state_save: tauri::State<MutexSave>) -> Result<SaveData, ()> {
    let mut save_option = state_save.inner().data.lock().unwrap();
    let save = save_option.as_mut().unwrap();

    save.inventory.edit_item(&mut save.file, index, value);
    Ok(save.clone())
}

#[tauri::command]
fn save(path: String, state_save: tauri::State<MutexSave>) -> Option<bool> {
    let mut save_option = state_save.inner().data.lock().unwrap();
    let save = save_option.as_mut().unwrap();

    match save.file.save(&path) {
        Ok(_) => Some(true),
        Err(_) => None
    }
}
