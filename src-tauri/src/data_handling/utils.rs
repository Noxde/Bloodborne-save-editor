#[cfg(test)]
pub mod test_utils {
    use crate::data_handling::{save::SaveData,
                file::FileData};
    use std::path::PathBuf;

    pub fn build_save_data(save_path: &str) -> SaveData {
        SaveData::build(&format!("saves/{}", save_path), PathBuf::from("resources")).unwrap()
    }

    pub fn build_file_data(save_path: &str) -> FileData {
        FileData::build(&format!("saves/{}", save_path), PathBuf::from("resources")).unwrap()
    }

    pub fn check_bytes(file_data: &FileData,index: usize,bytes: &[u8]) -> bool {
        let mut equal = true;
        for (i, byte) in bytes.iter().enumerate() {
            if file_data.bytes[index+i]!=*byte {
                equal = false;
                break;
            }
        }
        if equal == false {
            println!("check_bytes failed:");
            for (i, byte) in bytes.iter().enumerate() {
                let equal = file_data.bytes[index+i] == *byte;
                println!("File byte: {:#02x}, test byte: {:#02x}, match: {}", file_data.bytes[index+i], *byte, equal);
            }
        }
        equal
    }
}
