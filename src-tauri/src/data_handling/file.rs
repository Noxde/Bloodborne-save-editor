use serde::{Deserialize, Serialize};

use super::{
    constants::{USERNAME_TO_AOB, USERNAME_TO_ISZ_GLITCH},
    enums::{Error, Location, TypeFamily},
    offsets::Offsets,
};
use std::{
    fs,
    io::{self, Read},
    path::PathBuf,
};

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

        // Read the entire file into a vector of bytes
        let mut bytes = Vec::new();
        file.read_to_end(&mut bytes).map_err(Error::IoError)?;

        if bytes.is_empty() {
            return Err(Error::CustomError("The selected file is empty."));
        }

        //Search the offsets
        let offsets = Offsets::build(&bytes)?;

        // Create a backup
        let backup_path = format!("{}.bak", path);
        fs::copy(path, backup_path).map_err(Error::IoError)?;

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

    pub fn get_flag(&self, offset_from_aob: usize) -> u8 {
        let value_offset = self.offsets.username + USERNAME_TO_AOB + offset_from_aob;

        self.bytes[value_offset]
    }

    pub fn set_flag(&mut self, offset_from_aob: usize, new_value: u8) {
        let value_offset = self.offsets.username + USERNAME_TO_AOB + offset_from_aob;

        self.bytes[value_offset] = new_value;
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

    pub fn find_article_offset(
        &self,
        index: u8,
        id: u32,
        type_family: TypeFamily,
        is_storage: bool,
    ) -> Option<usize> {
        let found = |offset| -> bool {
            let last_byte = match type_family {
                TypeFamily::Armor | TypeFamily::Item => 0x00,
                TypeFamily::Weapon => self.bytes[offset + 11],
            };
            let current_id = u32::from_le_bytes([
                self.bytes[offset + 8],
                self.bytes[offset + 9],
                self.bytes[offset + 10],
                last_byte,
            ]);
            (index == self.bytes[offset]) && (id == current_id)
        };

        let (inv, key) = match is_storage {
            true => (self.offsets.storage, self.offsets.key_inventory),
            false => (self.offsets.inventory, self.offsets.key_inventory),
        };

        //Search for the article in the inventory
        let mut i = inv.0;
        while (i <= inv.1 - 24) && (!found(i)) {
            i += 16;
        }

        //If the article wasnt found, search for it in the key inventory
        if !found(i) {
            i = key.0;
            while (i <= key.1 - 16) && (!found(i)) {
                i += 16;
            }
        }

        match found(i) {
            true => Some(i),
            false => None,
        }
    }

    pub fn find_upgrade_offset(&self, id: u32) -> Option<usize> {
        //Search for the upgrade
        for i in (self.offsets.upgrades.0..self.offsets.upgrades.1).step_by(40) {
            let current_id = u32::from_le_bytes([
                self.bytes[i + 0],
                self.bytes[i + 1],
                self.bytes[i + 2],
                self.bytes[i + 3],
            ]);
            if id == current_id {
                return Some(i);
            }
        }
        None
    }

    //If there is an empty slot return the index of the first byte of the first part
    //Or else return None
    pub fn find_inv_empty_slot(&self, location: Location) -> Option<usize> {
        let (start, end) = match location {
            Location::Inventory => self.offsets.inventory,
            Location::Storage => self.offsets.storage,
        };
        let empty = [0, 0, 0, 0, 0xFF, 0xFF, 0xFF, 0xFF, 0, 0, 0, 0];
        let mut buffer = [0; 12];
        //-4 so it doesn't match the last slot
        for i in (start..end - 4).step_by(16) {
            buffer.copy_from_slice(&self.bytes[i + 4..=i + 15]);
            if empty == buffer {
                return Some(i + 4); //First byte of the first part of the slot
            }
        }
        None
    }

    pub fn get_playtime(&self) -> u32 {
        let time_bytes = [
            self.bytes[0x08],
            self.bytes[0x09],
            self.bytes[0x0A],
            self.bytes[0x0B],
        ];
        let time_ms = u32::from_le_bytes(time_bytes);

        time_ms
    }

    pub fn set_playtime(&mut self, new_playtime: [u8; 4]) {
        for (i, j) in (0x08..=0x0B).enumerate() {
            self.bytes[j] = new_playtime[i];
        }
    }

    pub fn get_isz(&self) -> [u8; 2] {
        return [
            self.bytes[USERNAME_TO_ISZ_GLITCH + self.offsets.username],
            self.bytes[USERNAME_TO_ISZ_GLITCH + self.offsets.username + 1],
        ];
    }

    pub fn fix_isz(&mut self) -> String {
        let values = self.get_isz();
        if values[0] == 0xFF {
            if values[1] < 0xC0 {
                self.bytes[USERNAME_TO_ISZ_GLITCH + self.offsets.username + 1] = 0x30;
                return "Partial Isz glitch fix applied".to_string();
            } else if values[1] == 0xC0 {
                self.bytes[USERNAME_TO_ISZ_GLITCH + self.offsets.username + 1] = 0xFF;
                return "Full Isz glitch fix applied".to_string();
            }
        }

        "No Isz glitch, no changes have been made".to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
        assert_eq!(
            file_data
                .find_article_offset(4, 1200, TypeFamily::Item, true)
                .unwrap(),
            0x10f28
        );
    }

    #[test]
    fn test_find_inv_empty_slot() {
        let file_data = FileData::build("saves/testsave4", PathBuf::from("resources")).unwrap();
        assert_eq!(
            file_data.find_inv_empty_slot(Location::Inventory).unwrap(),
            0xcfb8
        );
        assert_eq!(
            file_data.find_inv_empty_slot(Location::Storage).unwrap(),
            0x15304
        );

        let file_data = FileData::build("saves/testsave0", PathBuf::from("resources")).unwrap();
        assert!(file_data.find_inv_empty_slot(Location::Inventory).is_none());
    }
}
