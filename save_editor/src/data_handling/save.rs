use super::player::PlayerData;
use std::{fmt, fs, io::{self, Read}};

#[derive(Debug)]
pub enum Error {
    IoError(io::Error),
    CustomError(&'static str),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::IoError(err) => write!(f, "I/0 error: {}",err),
            Error::CustomError(err) => write!(f, "Save error: {}",err),
        }
    }
}

impl std::error::Error for Error {}

pub struct SaveData {
    pub player: PlayerData,
}

impl SaveData {
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

    pub fn build(path: &str, username: &str) -> Result<SaveData, Error> {
        // Open the save file
        let mut file = fs::File::open(path).map_err(Error::IoError)?;

        // Read the entire file into a vector of bytes
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer).map_err(Error::IoError)?;

        let username_offset = SaveData::search_username(username, &buffer)
            .map_err(Error::CustomError)?;

        Ok(SaveData{player: PlayerData::new(&buffer, username_offset)})
    }
}

