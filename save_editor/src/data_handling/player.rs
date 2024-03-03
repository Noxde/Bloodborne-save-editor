use serde::Deserialize;

use super::file::FileData;
use std::fs::File;
use std::io::{self, BufReader};

#[derive(Deserialize, Debug)]
pub struct Stat {
    pub name: String,
    pub rel_offset: isize,
    pub lenght: usize,
    pub times: usize,
    pub value: u32,
}

pub struct PlayerData {
    pub stats: Vec<Stat>,
}

impl PlayerData {
    pub fn new(file: &FileData) -> Result<PlayerData, io::Error> {
        let json_file = File::open("offsets.json")?;
        let reader = BufReader::new(json_file);

        // Read the JSON contents of the file as Vec<Stat>.
        let mut stats: Vec<Stat> = serde_json::from_reader(reader)?;
        for s in &mut stats {
            s.value = file.get_number(s.rel_offset, s.lenght);
        }

        Ok(PlayerData { stats })
    }
}
