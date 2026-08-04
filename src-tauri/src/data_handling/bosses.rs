use super::file::FileData;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{self, BufReader};

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

pub fn new(file: &FileData) -> Result<Vec<Boss>, io::Error> {
    let bosses_str = include_str!("../../resources/bosses.json");

    let mut bosses: Vec<Boss> = serde_json::from_str(bosses_str)?;
    for b in &mut bosses {
        for f in &mut b.flags {
            f.current_value = file.get_flag(f.rel_offset);
        }
    }

    Ok(bosses)
}
