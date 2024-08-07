use serde::Serialize;

use super::enums::Error;
use std::{
    fs,
    io::{self, Read},
};

#[derive(Serialize)]
pub struct FileData {
    pub bytes: Vec<u8>,
    pub username_offset: usize,
}

impl FileData {
    fn search_username(username: &str, save_data: &Vec<u8>) -> Result<usize, &'static str> {
        let mut matchs: Vec<usize> = Vec::new();
        let mut encoded_username = Vec::new();

        //Encodes the username
        for byte in username.as_bytes() {
            encoded_username.push(0);
            encoded_username.push(*byte);
        }
        let username_len = encoded_username.len();
        let save_len = save_data.len();

        if save_len>username_len {  //TODO: Compare save_len with a more appropriate value
            //Searches for the username
            for i in 0..(save_data.len() - username_len) {
                if *encoded_username == save_data[i..(i + username_len)] {
                    matchs.push(i);
                }
            }
        } 

        //The last match is the valid one
        if matchs.len() == 0 {
            Err("Failed to find username in save data.")
        } else {
            Ok(*matchs.first().unwrap())
        }
    }

    pub fn build(path: &str, username: &str) -> Result<FileData, Error> {
        // Open the save file
        let mut file = fs::File::open(path).map_err(Error::IoError)?;

        // Create a backup
        let backup_path = format!("{}.bak", path);
        fs::copy(path, backup_path).map_err(Error::IoError)?;

        // Read the entire file into a vector of bytes
        let mut bytes = Vec::new();
        file.read_to_end(&mut bytes).map_err(Error::IoError)?;

        let username_offset =
            FileData::search_username(username, &bytes).map_err(Error::CustomError)?;

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
