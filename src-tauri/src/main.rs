// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::error::Error;
mod data_handling;

use data_handling::save::SaveData;

fn main() -> Result<(), Box<dyn Error>> {

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![make_save])
        .run(tauri::generate_context!())?;

    Ok(())
}

#[tauri::command]
fn make_save(path: &str, name: &str) -> Option<SaveData> {
    match SaveData::build(path, name) {
        Ok(s) => Some(s),
        Err(_) => None
    }
}
