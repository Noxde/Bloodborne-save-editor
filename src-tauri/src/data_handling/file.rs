use serde::{Deserialize, Serialize};
use super::{constants::{USERNAME_TO_INV_OFFSET, USERNAME_TO_KEY_INV_OFFSET}, enums::{Error, TypeFamily}};
use std::{
    fs,
    io::{self, Read},
    path::PathBuf,
};

#[derive(Serialize, Deserialize, Clone)]
pub struct Offsets {
    pub username: usize, //Beginning
    pub inventory: (usize, usize), //Beginning and end
    pub key_inventory: (usize, usize), //Beginning and end
}

impl Offsets {
    //Searches the username and inventories offsets
    fn build(bytes: &Vec<u8>) -> Result<Offsets, Error> {
        let mut username_offset = 0;
        let mut inventory_offset = (0, 0);
        let mut key_inventory_offset = (0, 0);
        let inv_start_bytes = vec![0x40, 0xf0, 0xff, 0xff]; //Bytes the inventory starts with
        let inv_start_bytes_len = inv_start_bytes.len();

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
        let find_end = |start: usize| -> Result<usize, Error> {
            let mut buffer = [0; 12];
            for i in (start .. bytes.len() - 15).step_by(16) {
                buffer.copy_from_slice(&bytes[i + 4 ..= i + 15]);
                if end == buffer {
                    return Ok(i + 15);
                }
            }
            Err(Error::CustomError("Failed to find the end of the inventory."))
        };
        inventory_offset.1 = find_end(inventory_offset.0)?;
        key_inventory_offset.1 = find_end(key_inventory_offset.0)?;

        Ok(Offsets {
            username: username_offset,
            inventory: inventory_offset,
            key_inventory: key_inventory_offset,
        })
    }
}

#[derive(Serialize, Deserialize, Clone)]
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

    pub fn find_article_offset(&self, index: u8, id: u32, type_family: TypeFamily) -> Option<usize> {
        let found = |offset| -> bool {
            let last_byte = match type_family {
                TypeFamily::Armor | TypeFamily::Item => 0x00,
                TypeFamily::Weapon => self.bytes[offset+11],
                TypeFamily::Upgrade => panic!("ERROR: Article cannot be an upgrade."),
            };
            let current_id = u32::from_le_bytes([self.bytes[offset+8],
                                                     self.bytes[offset+9],
                                                     self.bytes[offset+10],
                                                     last_byte]);
            (index == self.bytes[offset]) && (id == current_id)
        };

        //Search for the article in the inventory
        let mut i = self.offsets.inventory.0;
        while (i <= self.offsets.inventory.1 - 16) && (!found(i)) {
            i+=16;
        }

        //If the article wasnt found, search for it in the key inventory
        if !found(i) {
            i = self.offsets.key_inventory.0;
            while (i <= self.offsets.key_inventory.1 - 16) && (!found(i)) {
                i+=16;
            }
        }

        match found(i) {
            true => Some(i),
            false => None,
        }
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

        //testsave0
        let file_data = FileData::build("saves/testsave0", PathBuf::from("resources")).unwrap();
        assert_eq!(file_data.offsets.username, 0x8777);
        assert_eq!(file_data.offsets.inventory, (0x894c, 0x8cdb));
        assert_eq!(file_data.offsets.key_inventory, (0x10540, 0x105af));

        //testsave1
        let file_data = FileData::build("saves/testsave1", PathBuf::from("resources")).unwrap();
        assert_eq!(file_data.offsets.username, 0xa82b);
        assert_eq!(file_data.offsets.inventory, (0xaa00, 0xb6af));
        assert_eq!(file_data.offsets.key_inventory, (0x125f4, 0x126e3));

        //testsave2
        let file_data = FileData::build("saves/testsave2", PathBuf::from("resources")).unwrap();
        assert_eq!(file_data.offsets.username, 0xa86f);
        assert_eq!(file_data.offsets.inventory, (0xaa44, 0xb643));
        assert_eq!(file_data.offsets.key_inventory, (0x12638, 0x12797));

        //testsave3
        let file_data = FileData::build("saves/testsave3", PathBuf::from("resources")).unwrap();
        assert_eq!(file_data.offsets.username, 0xb473);
        assert_eq!(file_data.offsets.inventory, (0xb648, 0xc8b7));
        assert_eq!(file_data.offsets.key_inventory, (0x1323c, 0x133db));

        //testsave4
        let file_data = FileData::build("saves/testsave4", PathBuf::from("resources")).unwrap();
        assert_eq!(file_data.offsets.username, 0xc85f);
        assert_eq!(file_data.offsets.inventory, (0xca34, 0xcfc3));
        assert_eq!(file_data.offsets.key_inventory, (0x14628, 0x14857));
    }
}
