use super::file::FileData;

pub struct PlayerData {
    pub health: u32,
}

impl PlayerData {
    pub fn new(file: &FileData) -> PlayerData {

        let health = file.get_number(-147, 4);

        PlayerData { health, }
    }
}