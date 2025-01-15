use serde::{Deserialize, Serialize};
use super::{constants::*, enums::{Error, TypeFamily}};
use std::{fs, io::{self, Read}, path::PathBuf};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Offsets {
    pub username: usize, //Beginning
    pub inventory: (usize, usize), //Beginning and end
    pub storage: (usize, usize), //Beginning and end
    pub upgrades: (usize, usize), //Beginning and end
    pub key_inventory: (usize, usize), //Beginning and end
    pub appearance: (usize, usize), //Beginning
    pub equipped_gems: (usize, usize), //Beginning and end
}

impl Offsets {
    //Searches the username and inventories offsets
    fn build(bytes: &Vec<u8>) -> Result<Offsets, Error> {
        let mut username_offset = 0;
        let mut inventory_offset = (0, 0);
        let mut upgrades_offset = (START_TO_UPGRADE, 0);
        let mut key_inventory_offset = (0, 0);
        let mut appearance_offset = (0, 0);
        let inv_start_bytes = vec![0x40, 0xf0, 0xff, 0xff]; //Bytes the inventory starts with
        let inv_start_bytes_len = inv_start_bytes.len();
        let appearance_start_bytes = [b'F', b'A', b'C', b'E'];

        let gems = [
            [0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00],
            [0x01, 0x00, 0x00, 0x00, 0x02, 0x00, 0x00, 0x00],
            [0x01, 0x00, 0x00, 0x00, 0x04, 0x00, 0x00, 0x00],
            [0x01, 0x00, 0x00, 0x00, 0x08, 0x00, 0x00, 0x00],
            [0x01, 0x00, 0x00, 0x00, 0x3f, 0x00, 0x00, 0x00]
          ];
        let runes =  [
            [0x02, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00],
            [0x02, 0x00, 0x00, 0x00, 0x02, 0x00, 0x00, 0x00]
        ];

        //Get the end offset for the upgrades
        for i in (upgrades_offset.0..(bytes.len())).step_by(40) {
            let current = &bytes[(i+8)..(i+16)];

            let is_match = runes.iter().any(|&x| current == x) || gems.iter().any(|&x| current == x);

            if !is_match {
                upgrades_offset.1 = i -1;
                break;
            }
        }
        //Searches for the inv_start_bytes
        for i in 0..(bytes.len() - inv_start_bytes_len) {
            if *inv_start_bytes == bytes[i..(i + inv_start_bytes_len)] {
                //If the beginning of the inventory is found we calculate the username_offset
                //and the beginning of the key_inventory
                inventory_offset.0 = i;
                username_offset = i - USERNAME_TO_INV_OFFSET;
                key_inventory_offset.0 = username_offset + USERNAME_TO_KEY_INV_OFFSET;
                break;
            }
        }

        if inventory_offset.0 == 0 {
            return Err(Error::CustomError("Failed to find username in save data."));
        }

        //Find the end of the inventories
        let end = [0, 0, 0, 0, 0xFF, 0xFF, 0xFF, 0xFF, 0, 0, 0, 0];
        let mut end_offset: Option<usize> = None;
        let mut empty_slots: usize = 0;
        let mut find_end = |start: usize, allow_empty: bool| -> Result<usize, Error> {
            //Maximum length of the normal inv before it reaches the key inv
            let inv_max_length = USERNAME_TO_KEY_INV_OFFSET - USERNAME_TO_INV_OFFSET;
            let inv_max_length = if bytes.len() < inv_max_length + start {bytes.len()-start} else {inv_max_length};
            let mut buffer = [0; 12];
            for i in (start ..  start + inv_max_length - 15).step_by(16) {
                buffer.copy_from_slice(&bytes[i + 4 ..= i + 15]);
                if end == buffer {
                    empty_slots += 1;
                    if end_offset.is_none() {
                        if !allow_empty {
                            return Ok(i + 15);
                        }
                        end_offset = Some(i + 15);
                    }
                    if empty_slots > MAX_EMPTY_INV_SLOTS {
                        return Ok(end_offset.unwrap());
                    }
                } else if end_offset.is_some() {
                    end_offset = None;
                    empty_slots = 0;
                }
            }
            match end_offset {
                Some(off) => Ok(off),
                None => Err(Error::CustomError("Failed to find the end of the inventory.")),
            }
        };
        //11 is subtracted to the offset to match the first byte of the first part of the next slot the game will open
        inventory_offset.1 = find_end(inventory_offset.0, true)? - 11;
        key_inventory_offset.1 = find_end(key_inventory_offset.0, false)?;

        //Searches for the appearance_start_bytes
        for i in 0xF000..(bytes.len() - 4) { //0xF000: In all the saves i found the save bytes after 0x10000
            if appearance_start_bytes == bytes[i..i + 4] {
                appearance_offset.0 = i + 4;
                appearance_offset.1 = appearance_offset.0 + APPEARANCE_BYTES_AMOUNT - 1;
                break;
            }
        }
        if appearance_offset.0 == 0 {
            return Err(Error::CustomError("Failed to find the appearance."));
        }

        let storage_start_offset = inventory_offset.0 + INV_TO_STORAGE_OFFSET;
        //11 is subtracted to the offset to match the first byte of the first part of the next slot the game will open
        let storage_offset = (storage_start_offset, find_end(storage_start_offset, true)? - 11);

        Ok(Offsets {
            username: username_offset,
            inventory: inventory_offset,
            storage: storage_offset,
            upgrades: upgrades_offset,
            key_inventory: key_inventory_offset,
            appearance: appearance_offset,
            equipped_gems: (0, 0),
        })
    }
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct FileData {
    pub bytes: Vec<u8>,
    pub offsets: Offsets,
    pub resources_path: PathBuf, //This is here for convenience
}

impl FileData {
    pub fn build(path: &str, resources_path: PathBuf) -> Result<FileData, Error> {
        // Open the save file
        let mut file = fs::File::open(path).map_err(Error::IoError)?;

        // Create a backup
        let backup_path = format!("{}.bak", path);
        fs::copy(path, backup_path).map_err(Error::IoError)?;

        // Read the entire file into a vector of bytes
        let mut bytes = Vec::new();
        file.read_to_end(&mut bytes).map_err(Error::IoError)?;

        if bytes.is_empty() {
            return Err(Error::CustomError("The selected file is empty."));
        }

        //Search the offsets
        let offsets = Offsets::build(&bytes)?;

        Ok(FileData {
            bytes,
            offsets,
            resources_path,
        })
    }

    //offset_from_username is value_offset-username_offset
    pub fn get_number(&self, offset_from_username: isize, length: usize) -> u32 {
        let value_offset = (self.offsets.username as isize + offset_from_username) as usize;
        let value_bytes = &self.bytes[value_offset..value_offset + length];

        let mut value: u32 = 0;
        let base: u32 = 256;

        for (index, byte) in value_bytes.iter().enumerate().rev() {
            value += *byte as u32 * (base.pow(index as u32));
        }

        value
    }

    pub fn edit(&mut self, rel_offset: isize, length: usize, times: usize, value: u32) {
        let value_bytes = value.to_le_bytes();
        let from_offset = (self.offsets.username as isize + rel_offset) as usize;

        for i in 0..times {
            let offset = i * 4;
            for (j, b) in value_bytes[..length].iter().enumerate() {
                self.bytes[from_offset + j + offset] = *b;
            }
        }
    }

    pub fn save(&self, path: &str) -> Result<(), io::Error> {
        fs::write(path, &self.bytes)
    }

    pub fn find_article_offset(&self, index: u8, id: u32, type_family: TypeFamily, is_storage: bool) -> Option<usize> {
        let found = |offset| -> bool {
            let last_byte = match type_family {
                TypeFamily::Armor | TypeFamily::Item => 0x00,
                TypeFamily::Weapon => self.bytes[offset+11],
            };
            let current_id = u32::from_le_bytes([self.bytes[offset+8],
                                                     self.bytes[offset+9],
                                                     self.bytes[offset+10],
                                                     last_byte]);
            (index == self.bytes[offset]) && (id == current_id)
        };

        let (inv, key) = match is_storage {
            true => (self.offsets.storage, self.offsets.key_inventory),
            false => (self.offsets.inventory, self.offsets.key_inventory)
        };

        //Search for the article in the inventory
        let mut i = inv.0;
        while (i <= inv.1 - 24) && (!found(i)) {
            i+=16;
        }

        //If the article wasnt found, search for it in the key inventory
        if !found(i) {
            i = key.0;
            while (i <= key.1 - 16) && (!found(i)) {
                i+=16;
            }
        }

        match found(i) {
            true => Some(i),
            false => None,
        }
    }

    pub fn find_upgrade_offset(&self, id: u32) -> Option<usize> {
        //Search for the upgrade
        for i in (self.offsets.upgrades.0 .. self.offsets.upgrades.1).step_by(40) {
            let current_id = u32::from_le_bytes([self.bytes[i+0],
                                                 self.bytes[i+1],
                                                 self.bytes[i+2],
                                                 self.bytes[i+3]]);
            if id == current_id {
                return Some(i);
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn offsets_build() {
        //Test with invalid path
        let file_data = FileData::build("invalid", PathBuf::from("resources"));
        assert!(file_data.is_err());
        if let Err(e) = file_data {
            assert!(e.to_string().contains("I/0 error:"));
        }

        //Test with empty save
        let file_data = FileData::build("saves/emptysave", PathBuf::from("resources"));
        assert!(file_data.is_err());
        if let Err(e) = file_data {
            assert_eq!(e.to_string(), "Save error: The selected file is empty.");
        }

        //Test with a save that has no inventory
        let file_data = FileData::build("saves/no_inv_save", PathBuf::from("resources"));
        assert!(file_data.is_err());
        if let Err(e) = file_data {
            assert_eq!(e.to_string(), "Save error: Failed to find username in save data.");
        }

        //Test a save in which the inventory has no end
        let file_data = FileData::build("saves/no_inv_end_save", PathBuf::from("resources"));
        assert!(file_data.is_err());
        if let Err(e) = file_data {
            assert_eq!(e.to_string(), "Save error: Failed to find the end of the inventory.");
        }

        //Test a save with no appearance
        let file_data = FileData::build("saves/noappearancesave0", PathBuf::from("resources"));
        assert!(file_data.is_err());
        if let Err(e) = file_data {
            assert_eq!(e.to_string(), "Save error: Failed to find the appearance.");
        }

        //testsave0
        let file_data = FileData::build("saves/testsave0", PathBuf::from("resources")).unwrap();
        assert_eq!(file_data.offsets.username, 0x8777);
        assert_eq!(file_data.offsets.inventory, (0x894c, 0x8cd0));
        assert_eq!(file_data.offsets.key_inventory, (0x10540, 0x105af));
        assert_eq!(file_data.offsets.upgrades, (84, 163));
        assert_eq!(file_data.offsets.appearance, (0x10e3c, 0x10e3c + APPEARANCE_BYTES_AMOUNT - 1));

        //testsave1
        let file_data = FileData::build("saves/testsave1", PathBuf::from("resources")).unwrap();
        assert_eq!(file_data.offsets.username, 0xa82b);
        assert_eq!(file_data.offsets.inventory, (0xaa00, 0xb6a4));
        assert_eq!(file_data.offsets.key_inventory, (0x125f4, 0x126e3));
        assert_eq!(file_data.offsets.upgrades, (84, 0x8c3));
        assert_eq!(file_data.offsets.appearance, (0x12ef0, 0x12ef0 + APPEARANCE_BYTES_AMOUNT - 1));

        //testsave2
        let file_data = FileData::build("saves/testsave2", PathBuf::from("resources")).unwrap();
        assert_eq!(file_data.offsets.username, 0xa86f);
        assert_eq!(file_data.offsets.inventory, (0xaa44, 0xb638));
        assert_eq!(file_data.offsets.key_inventory, (0x12638, 0x12797));
        assert_eq!(file_data.offsets.upgrades, (84, 0x7d3));
        assert_eq!(file_data.offsets.appearance, (0x12f34, 0x12f34 + APPEARANCE_BYTES_AMOUNT - 1));

        //testsave3
        let file_data = FileData::build("saves/testsave3", PathBuf::from("resources")).unwrap();
        assert_eq!(file_data.offsets.username, 0xb473);
        assert_eq!(file_data.offsets.inventory, (0xb648, 0xc8ac));
        assert_eq!(file_data.offsets.key_inventory, (0x1323c, 0x133db));
        assert_eq!(file_data.offsets.upgrades, (84, 0xf7b));
        assert_eq!(file_data.offsets.appearance, (0x13b38, 0x13b38 + APPEARANCE_BYTES_AMOUNT - 1));

        //testsave4
        let file_data = FileData::build("saves/testsave4", PathBuf::from("resources")).unwrap();
        assert_eq!(file_data.offsets.username, 0xc85f);
        assert_eq!(file_data.offsets.inventory, (0xca34, 0xe778));
        assert_eq!(file_data.offsets.key_inventory, (0x14628, 0x14857));
        assert_eq!(file_data.offsets.upgrades, (84, 163));
        assert_eq!(file_data.offsets.appearance, (0x14f24, 0x14f24 + APPEARANCE_BYTES_AMOUNT - 1));

        //testsave8
        let file_data = FileData::build("saves/testsave8", PathBuf::from("resources")).unwrap();
        assert_eq!(file_data.offsets.username, 0x19897);
        assert_eq!(file_data.offsets.inventory, (0x19a6c, 0x210d0));
        assert_eq!(file_data.offsets.key_inventory, (0x21660, 0x218ef));
        assert_eq!(file_data.offsets.upgrades, (84, 0x10ae3));
        assert_eq!(file_data.offsets.appearance, (0x21f5c, 0x21f5c + APPEARANCE_BYTES_AMOUNT - 1));
    }

    #[test]
    fn test_find_upgrade_offset() {
        //testsave3
        let file_data = FileData::build("saves/testsave3", PathBuf::from("resources")).unwrap();

        //Does not exist
        assert_eq!(file_data.find_upgrade_offset(0xFFFFFFFF), None);
        //First one
        assert_eq!(file_data.find_upgrade_offset(0xC08006B5), Some(0x54));
        //Last one
        assert_eq!(file_data.find_upgrade_offset(0xC0800715), Some(0xF54));
        //Other ones
        assert_eq!(file_data.find_upgrade_offset(0xC08006E5), Some(0x7D4));
        assert_eq!(file_data.find_upgrade_offset(0xC08006D4), Some(0x52C));
        assert_eq!(file_data.find_upgrade_offset(0xC08006C2), Some(0x25C));
        assert_eq!(file_data.find_upgrade_offset(0x00000000), None);
    }

    #[test]
    fn test_file_data_save() {
        let mut file_data = FileData::build("saves/testsave0", PathBuf::from("resources")).unwrap();
        file_data.edit(500, 2, 10, 500);
        file_data.save("saves/savetestsave").unwrap();
        let file_data2 = FileData::build("saves/savetestsave", PathBuf::from("resources")).unwrap();
        assert_eq!(file_data, file_data2);
    }

    #[test]
    fn test_find_article_offset() {
        let file_data = FileData::build("saves/testsave0", PathBuf::from("resources")).unwrap();
        assert_eq!(file_data.find_article_offset(4, 1200, TypeFamily::Item, true).unwrap(), 0x10f28);
    }
}
