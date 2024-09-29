use serde::{Deserialize, Serialize};

use super::file::FileData;
use std::fs::File;
use std::io::{self, BufReader};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
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
    let file_path = file.resources_path.join("offsets.json");
    let json_file =  File::open(file_path)?;
    let reader = BufReader::new(json_file);

    // Read the JSON contents of the file as Vec<Stat>.
    let mut stats: Vec<Stat> = serde_json::from_reader(reader)?;
    for s in &mut stats {
        s.value = file.get_number(s.rel_offset, s.length);
    }

    Ok(stats)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_new() {
        //testsave0
        let file_data = FileData::build("saves/testsave0", PathBuf::from("resources")).unwrap();
        let stats = new(&file_data).unwrap();
        assert_eq!(stats[0], Stat {
            name: "Health".to_string(),
            rel_offset: -147,
            length: 4,
            times:3,
            value: 1900,
        });
        assert_eq!(stats[1], Stat {
            name: "Stamina".to_string(),
            rel_offset: -119,
            length: 4,
            times:3,
            value: 91,
        });
        assert_eq!(stats[2], Stat {
            name: "Echoes".to_string(),
            rel_offset: -19,
            length: 4,
            times:1,
            value: 9987417,
        });
        assert_eq!(stats[3], Stat {
            name: "Insight".to_string(),
            rel_offset: -35,
            length: 4,
            times:1,
            value: 999,
        });
        assert_eq!(stats[4], Stat {
            name: "Level".to_string(),
            rel_offset: -23,
            length: 4,
            times:1,
            value: 594,
        });
        assert_eq!(stats[5], Stat {
            name: "Vitality".to_string(),
            rel_offset: -103,
            length: 1,
            times:1,
            value: 99,
        });
        assert_eq!(stats[6], Stat {
            name: "Endurance".to_string(),
            rel_offset: -95,
            length: 1,
            times:1,
            value: 99,
        });
        assert_eq!(stats[7], Stat {
            name: "Strength".to_string(),
            rel_offset: -79,
            length: 1,
            times:1,
            value: 99,
        });
        assert_eq!(stats[8], Stat {
            name: "Skill".to_string(),
            rel_offset: -71,
            length: 1,
            times:1,
            value: 99,
        });
        assert_eq!(stats[9], Stat {
            name: "Bloodtinge".to_string(),
            rel_offset: -63,
            length: 1,
            times:1,
            value: 99,
        });
        assert_eq!(stats[10], Stat {
            name: "Arcane".to_string(),
            rel_offset: -55,
            length: 1,
            times:1,
            value: 99,
        });
    }

    #[test]
    fn stat_edit() {
        //testsave0
        let mut file_data = FileData::build("saves/testsave0", PathBuf::from("resources")).unwrap();
        let mut stats = new(&file_data).unwrap();
        assert_eq!(stats[0].value, 1900);
        assert_eq!(stats[1].value, 91);
        assert_eq!(stats[2].value, 9987417);
        assert_eq!(stats[3].value, 999);
        assert_eq!(stats[4].value, 594);
        stats[0].edit(10, &mut file_data);
        stats[1].edit(20, &mut file_data);
        stats[2].edit(30, &mut file_data);
        stats[3].edit(40, &mut file_data);
        stats[4].edit(50, &mut file_data);
        assert_eq!(stats[0].value, 10);
        assert_eq!(stats[1].value, 20);
        assert_eq!(stats[2].value, 30);
        assert_eq!(stats[3].value, 40);
        assert_eq!(stats[4].value, 50);
    }
}
