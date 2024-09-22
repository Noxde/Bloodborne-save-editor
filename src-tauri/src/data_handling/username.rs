use super::file::FileData;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Username {
    pub string: String,
}

impl Username {
    pub fn build(file_data: &FileData) -> Username {
        let mut chars: Vec<char> = Vec::with_capacity(16);
        let start = file_data.offsets.username + 1;
        file_data.bytes[start .. start + 32].iter().step_by(2).take_while(|&c| *c!=0).for_each(|c| chars.push(*c as char));
        let string = chars.into_iter().collect();
        Username {
            string,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn username_build() {
        //testsave0
        let file_data = FileData::build("saves/testsave0", PathBuf::from("resources")).unwrap();
        let username = Username::build(&file_data);
        assert_eq!(username.string, String::from("Proyectito"));

        //testsave1
        let file_data = FileData::build("saves/testsave1", PathBuf::from("resources")).unwrap();
        let username = Username::build(&file_data);
        assert_eq!(username.string, String::from("Toe Taster"));

        //testsave2
        let file_data = FileData::build("saves/testsave2", PathBuf::from("resources")).unwrap();
        let username = Username::build(&file_data);
        assert_eq!(username.string, String::from("I'm Here To Help"));

        //testsave3
        let file_data = FileData::build("saves/testsave3", PathBuf::from("resources")).unwrap();
        let username = Username::build(&file_data);
        assert_eq!(username.string, String::from("Yeezy"));

        //testsave6
        let file_data = FileData::build("saves/testsave6", PathBuf::from("resources")).unwrap();
        let username = Username::build(&file_data);
        assert_eq!(username.string, String::from("Touch Me"));
    }
}
