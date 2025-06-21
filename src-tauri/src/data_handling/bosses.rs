use serde::{Deserialize, Serialize};
use super::file::FileData;
use std::fs::File;
use std::io::{self, BufReader};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Flag {
    rel_offset: usize,
    dead_value: u8,
    alive_value: u8,
    current_value: u8
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Boss {
    name: String,
    flags: Vec<Flag>
}

pub fn new(file: &FileData) -> Result<Vec<Boss>, io::Error> {
    let file_path = file.resources_path.join("bosses.json");
    let json_file =  File::open(file_path)?;
    let reader = BufReader::new(json_file);

    // Read the JSON contents of the file as Vec<Stat>.
    let mut bosses: Vec<Boss> = serde_json::from_reader(reader)?;
    for b in &mut bosses {
        for f in &mut b.flags {
            f.current_value = file.get_flag(f.rel_offset);
        }
    }

    Ok(bosses)
}