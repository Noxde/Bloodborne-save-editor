use super::{
    enums::Error,
    file::FileData,
    stats::{self, Stat},
};

pub struct SaveData {
    pub file: FileData,
    pub stats: Vec<Stat>,
}

impl SaveData {
    pub fn build(path: &str, username: &str) -> Result<SaveData, Error> {
        let file = FileData::build(path, username)?;

        let stats = stats::new(&file).unwrap();

        Ok(SaveData { file, stats })
    }
}
