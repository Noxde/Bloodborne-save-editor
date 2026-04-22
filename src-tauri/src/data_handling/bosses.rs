use super::file::FileData;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{self, BufReader};

use tauri::Manager;
use tauri_plugin_fs::FsExt;
use crate::BaseDirectory;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Flag {
    rel_offset: usize,
    dead_value: u8,
    alive_value: u8,
    current_value: u8,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Boss {
    name: String,
    flags: Vec<Flag>,
}

pub fn new(file: &FileData, handle: &tauri::AppHandle) -> Result<Vec<Boss>, io::Error> {
    let resource_path = handle.path().resolve("resources/bosses.json", BaseDirectory::Resource).unwrap();
    let json = handle.fs().read_to_string(&resource_path).unwrap();

    // Read the JSON contents of the file as Vec<Stat>.
    let mut bosses: Vec<Boss> = serde_json::from_str(&json)?;
    for b in &mut bosses {
        for f in &mut b.flags {
            f.current_value = file.get_flag(f.rel_offset);
        }
    }

    Ok(bosses)
}
