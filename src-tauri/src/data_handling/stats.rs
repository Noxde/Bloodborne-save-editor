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

// Health and Stamina are stored as three consecutive values: [current, max, base].
// `max` (and `current`) include rune bonuses such as Clockwise Metamorphosis, `base` does not.
// The editor works with `base` so rune bonuses never get baked into the character's stats (#14).
const POOL_SLOTS: usize = 3;
const SLOT_SIZE: isize = 4;

impl Stat {
    pub fn edit(&mut self, value: u32, file: &mut FileData) {
        //Updates the stat value and saves it in a FileData instance
        self.value = value;
        write_stat(file, self.rel_offset, self.length, self.times, self.value);
    }
}

fn is_pool(times: usize) -> bool {
    times == POOL_SLOTS
}

fn base_offset(rel_offset: isize) -> isize {
    rel_offset + (POOL_SLOTS as isize - 1) * SLOT_SIZE
}

pub fn new(file: &FileData) -> Result<Vec<Stat>, io::Error> {
    let offsets_str = include_str!("../../resources/offsets.json");

    // Read the JSON contents of the file as Vec<Stat>.
    let mut stats: Vec<Stat> = serde_json::from_str(offsets_str)?;
    for s in &mut stats {
        let offset = if is_pool(s.times) {
            base_offset(s.rel_offset)
        } else {
            s.rel_offset
        };
        s.value = file.get_number(offset, s.length);
    }

    Ok(stats)
}

/// Writes a stat to the save.
/// For Health and Stamina `value` is the base value: max keeps the rune bonus already
/// in the save (same max/base ratio) and current is refilled to the new max.
pub fn write_stat(file: &mut FileData, rel_offset: isize, length: usize, times: usize, value: u32) {
    if !is_pool(times) {
        file.edit(rel_offset, length, times, value);
        return;
    }

    let old_max = file.get_number(rel_offset + SLOT_SIZE, length) as u64;
    let old_base = file.get_number(base_offset(rel_offset), length) as u64;
    let new_max = if old_base == 0 {
        value
    } else {
        (value as u64 * old_max / old_base).min(u32::MAX as u64) as u32
    };

    file.edit(rel_offset, length, 2, new_max); // current and max
    file.edit(base_offset(rel_offset), length, 1, value);
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
        assert_eq!(
            stats[0],
            Stat {
                name: "Health".to_string(),
                rel_offset: -147,
                length: 4,
                times: 3,
                value: 1900,
            }
        );
        assert_eq!(
            stats[1],
            Stat {
                name: "Stamina".to_string(),
                rel_offset: -119,
                length: 4,
                times: 3,
                value: 91,
            }
        );
        assert_eq!(
            stats[2],
            Stat {
                name: "Echoes".to_string(),
                rel_offset: -19,
                length: 4,
                times: 1,
                value: 9987417,
            }
        );
        assert_eq!(
            stats[3],
            Stat {
                name: "Insight".to_string(),
                rel_offset: -35,
                length: 4,
                times: 1,
                value: 999,
            }
        );
        assert_eq!(
            stats[4],
            Stat {
                name: "Level".to_string(),
                rel_offset: -23,
                length: 4,
                times: 1,
                value: 594,
            }
        );
        assert_eq!(
            stats[5],
            Stat {
                name: "Vitality".to_string(),
                rel_offset: -103,
                length: 4,
                times: 1,
                value: 99,
            }
        );
        assert_eq!(
            stats[6],
            Stat {
                name: "Endurance".to_string(),
                rel_offset: -95,
                length: 4,
                times: 1,
                value: 99,
            }
        );
        assert_eq!(
            stats[7],
            Stat {
                name: "Strength".to_string(),
                rel_offset: -79,
                length: 4,
                times: 1,
                value: 99,
            }
        );
        assert_eq!(
            stats[8],
            Stat {
                name: "Skill".to_string(),
                rel_offset: -71,
                length: 4,
                times: 1,
                value: 99,
            }
        );
        assert_eq!(
            stats[9],
            Stat {
                name: "Bloodtinge".to_string(),
                rel_offset: -63,
                length: 4,
                times: 1,
                value: 99,
            }
        );
        assert_eq!(
            stats[10],
            Stat {
                name: "Arcane".to_string(),
                rel_offset: -55,
                length: 4,
                times: 1,
                value: 99,
            }
        );
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

    fn pool(file_data: &FileData, rel_offset: isize) -> [u32; 3] {
        [0, 1, 2].map(|i| file_data.get_number(rel_offset + i * SLOT_SIZE, 4))
    }

    #[test]
    fn test_new_reads_base_without_runes() {
        //testsave3: Health [current 1100, max 1100, base 1000] (+10% rune)
        let file_data = FileData::build("saves/testsave3", PathBuf::from("resources")).unwrap();
        let stats = new(&file_data).unwrap();
        assert_eq!(stats[0].value, 1000);

        //testsave8: Stamina [192, 192, 160] (+20% rune)
        let file_data = FileData::build("saves/testsave8", PathBuf::from("resources")).unwrap();
        let stats = new(&file_data).unwrap();
        assert_eq!(stats[1].value, 160);

        //testsave4: Health [current 2458, max 2500, base 2500] (damaged)
        let file_data = FileData::build("saves/testsave4", PathBuf::from("resources")).unwrap();
        let stats = new(&file_data).unwrap();
        assert_eq!(stats[0].value, 2500);
    }

    #[test]
    fn test_edit_pool_keeps_rune_bonus() {
        //testsave3: Health [1100, 1100, 1000]
        let mut file_data = FileData::build("saves/testsave3", PathBuf::from("resources")).unwrap();
        let mut stats = new(&file_data).unwrap();
        stats[0].edit(2000, &mut file_data);
        assert_eq!(stats[0].value, 2000);
        assert_eq!(pool(&file_data, -147), [2200, 2200, 2000]);
    }

    #[test]
    fn test_edit_pool_damaged_keeps_base() {
        //testsave4: Health [2458, 2500, 2500]. Rewriting the same base must not lower it
        let mut file_data = FileData::build("saves/testsave4", PathBuf::from("resources")).unwrap();
        let mut stats = new(&file_data).unwrap();
        let base = stats[0].value;
        stats[0].edit(base, &mut file_data);
        assert_eq!(pool(&file_data, -147), [2500, 2500, 2500]);
    }

    #[test]
    fn test_edit_single_stat_unchanged() {
        //Stats stored once are written as before
        let mut file_data = FileData::build("saves/testsave0", PathBuf::from("resources")).unwrap();
        write_stat(&mut file_data, -19, 4, 1, 12345);
        assert_eq!(file_data.get_number(-19, 4), 12345);
        assert_eq!(new(&file_data).unwrap()[2].value, 12345);
    }
}
