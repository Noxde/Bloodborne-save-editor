use serde::Deserialize;

use super::file::FileData;
use std::fs::File;
use std::io::{self, BufReader};

#[derive(Deserialize, Debug)]
pub struct Stat {
    pub name: String,
    pub rel_offset: isize,
    pub length: usize,
    pub times: usize,
    pub value: u32,
}

impl Stat {
    pub fn edit(&mut self, value: u32, file: &mut FileData) {
        //Updates the stat value and saves it in a FileData instance
        self.value = value;
        file.edit(self.rel_offset, self.length, self.times, self.value);
    }
}

pub fn new(file: &FileData) -> Result<Vec<Stat>, io::Error> {
    let json_file = File::open("offsets.json")?;
    let reader = BufReader::new(json_file);

    // Read the JSON contents of the file as Vec<Stat>.
    let mut stats: Vec<Stat> = serde_json::from_reader(reader)?;
    for s in &mut stats {
        s.value = file.get_number(s.rel_offset, s.length);
    }

    Ok(stats)
}