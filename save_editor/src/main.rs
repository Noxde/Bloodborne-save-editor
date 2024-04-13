// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::error::Error;

mod data_handling;

fn main() -> Result<(), Box<dyn Error>> {

    tauri::Builder::default()
        .run(tauri::generate_context!())?;

    Ok(())
}
