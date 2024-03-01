use super::enums::Error;
use std::{fs, io::Read};

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

        //Searches for the username
        for i in 0..(save_data.len()-username_len) {
            if *encoded_username == save_data[i..(i+username_len)] {
                matchs.push(i);
            }
        }

        //The last match is the valid one
        if matchs.len() == 0 {
            Err("Failed to find username in save data.")
        } else {
            Ok(*matchs.last().unwrap())
        }
    }

    pub fn build(path: &str, username: &str) -> Result<FileData, Error> {
        // Open the save file
        let mut file = fs::File::open(path).map_err(Error::IoError)?;

        // Read the entire file into a vector of bytes
        let mut bytes = Vec::new();
        file.read_to_end(&mut bytes).map_err(Error::IoError)?;

        let username_offset = FileData::search_username(username, &bytes)
            .map_err(Error::CustomError)?;

        Ok(FileData{bytes, username_offset,})
    }
}

