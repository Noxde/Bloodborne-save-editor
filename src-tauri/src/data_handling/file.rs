use serde::{Deserialize, Serialize};
use super::enums::Error;
use std::{
    fs,
    io::{self, Read},
};

//Distance between the username and the beginning of the inventory
const USERNAME_TO_INV_OFFSET: usize = 469;

#[derive(Serialize, Deserialize, Clone)]
pub struct FileData {
    pub bytes: Vec<u8>,
    pub username_offset: usize,
}

impl FileData {
    ///Searches the username in the file and, if found, returns it's index minus one.
    fn search_username(save_data: &Vec<u8>) -> Result<usize, &'static str> {
        let inv_start_bytes = vec![0x40, 0xf0, 0xff, 0xff]; //Bytes the inventory starts with

        let inv_start_bytes_len = inv_start_bytes.len();

        //Searches for the inv_start_bytes
        for i in 0..(save_data.len() - inv_start_bytes_len) {
            if *inv_start_bytes == save_data[i..(i + inv_start_bytes_len)] {
                    return Ok(i-USERNAME_TO_INV_OFFSET);
            }
        }
        Err("Failed to find username in save data.")
    }

    pub fn build(path: &str) -> Result<FileData, Error> {
        // Open the save file
        let mut file = fs::File::open(path).map_err(Error::IoError)?;

        // Create a backup
        let backup_path = format!("{}.bak", path);
        fs::copy(path, backup_path).map_err(Error::IoError)?;

        // Read the entire file into a vector of bytes
        let mut bytes = Vec::new();
        file.read_to_end(&mut bytes).map_err(Error::IoError)?;

        let username_offset =
            FileData::search_username(&bytes).map_err(Error::CustomError)?;

        Ok(FileData {
            bytes,
            username_offset,
        })
    }

    //offset_from_username is value_offset-username_offset
    pub fn get_number(&self, offset_from_username: isize, length: usize) -> u32 {
        let value_offset = (self.username_offset as isize + offset_from_username) as usize;
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
        let from_offset = (self.username_offset as isize + rel_offset) as usize;

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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_search_username() {
        //testsave0
        let file_data = FileData::build("saves/testsave0").unwrap();
        assert_eq!(file_data.username_offset, 34679);
        //Test with invalid path
        let file_data = FileData::build("invalid");
        assert!(file_data.is_err());
        if let Err(e) = file_data {
            assert_eq!(e.to_string(), "I/0 error: No such file or directory (os error 2)");
        }

        //testsave1
        let file_data = FileData::build("saves/testsave1").unwrap();
        assert_eq!(file_data.username_offset, 43051);

        //testsave2
        let file_data = FileData::build("saves/testsave2").unwrap();
        assert_eq!(file_data.username_offset, 43119);

        //testsave3
        let file_data = FileData::build("saves/testsave3").unwrap();
        assert_eq!(file_data.username_offset, 46195);

        //testsave4
        let file_data = FileData::build("saves/testsave4").unwrap();
        assert_eq!(file_data.username_offset, 51295);
    }
}
