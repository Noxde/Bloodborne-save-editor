use super::{
        enums::Error, 
        file::FileData, 
        player::PlayerData
    };

pub struct SaveData {
    pub file: FileData,
    pub player: PlayerData,
}

impl SaveData {
    pub fn build(path: &str, username: &str) -> Result<SaveData, Error> {

        let file = FileData::build(path, username)?;

        let player = PlayerData::new(&file);

        Ok(SaveData{file, player})
    }
}

