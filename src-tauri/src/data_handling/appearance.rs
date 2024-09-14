use super::{file::FileData,
            enums::Error};
use std::{fs::{self, File},
          io::Read};

pub fn export(file_data: &FileData, path: &str) -> Result<(), Error> {
    let mut export_bytes = Vec::new();
    export_bytes.extend_from_slice(&file_data.bytes[file_data.offsets.appearance.0 ..= file_data.offsets.appearance.1]);

    fs::write(path, &export_bytes).map_err(Error::IoError)
}

pub fn import(file_data: &mut FileData, path: &str) -> Result<(), Error> {
    // Read the exported file into a vector of bytes
    let mut file = File::open(path).map_err(Error::IoError)?;
    let mut bytes = Vec::new();

    file.read_to_end(&mut bytes).map_err(Error::IoError)?;
    if bytes.len() != 0xEB {
        return Err(Error::CustomError("Not correct size"));
    }
    let start = file_data.offsets.appearance;
    for i in start.0 ..= start.1 {
        file_data.bytes[i] = bytes[i - start.0];
    }

    Ok(())
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::data_handling::constants::APPEARANCE_BYTES_AMOUNT;
    use std::{path::PathBuf,
              fs::File,
              io::Read};

    #[test]
    fn test_export() {
        //TESTSAVE0
        let file_data = FileData::build("saves/testsave0", PathBuf::from("resources")).unwrap();
        export(&file_data, "saves/testexport0").unwrap();

        // Read the exported file into a vector of bytes
        let mut file = File::open("saves/testexport0").unwrap();
        let mut bytes = Vec::new();
        file.read_to_end(&mut bytes).unwrap();

        //Check the contents of the file
        assert_eq!(bytes.len(), APPEARANCE_BYTES_AMOUNT);
        let start = file_data.offsets.appearance;
        assert_eq!(bytes,  file_data.bytes[start.0 ..= start.1]);

        //TESTSAVE1
        let file_data = FileData::build("saves/testsave1", PathBuf::from("resources")).unwrap();
        export(&file_data, "saves/testexport1").unwrap();

        // Read the exported file into a vector of bytes
        let mut file = File::open("saves/testexport1").unwrap();
        let mut bytes = Vec::new();
        file.read_to_end(&mut bytes).unwrap();

        //Check the contents of the file
        assert_eq!(bytes.len(), APPEARANCE_BYTES_AMOUNT);
        let start = file_data.offsets.appearance;
        assert_eq!(bytes,  file_data.bytes[start.0 ..= start.1]);

        //TESTSAVE2
        let file_data = FileData::build("saves/testsave2", PathBuf::from("resources")).unwrap();
        export(&file_data, "saves/testexport2").unwrap();

        // Read the exported file into a vector of bytes
        let mut file = File::open("saves/testexport2").unwrap();
        let mut bytes = Vec::new();
        file.read_to_end(&mut bytes).unwrap();

        //Check the contents of the file
        assert_eq!(bytes.len(), APPEARANCE_BYTES_AMOUNT);
        let start = file_data.offsets.appearance;
        assert_eq!(bytes,  file_data.bytes[start.0 ..= start.1]);

        //TESTSAVE3
        let file_data = FileData::build("saves/testsave3", PathBuf::from("resources")).unwrap();
        export(&file_data, "saves/testexport3").unwrap();

        // Read the exported file into a vector of bytes
        let mut file = File::open("saves/testexport3").unwrap();
        let mut bytes = Vec::new();
        file.read_to_end(&mut bytes).unwrap();

        //Check the contents of the file
        assert_eq!(bytes.len(), APPEARANCE_BYTES_AMOUNT);
        let start = file_data.offsets.appearance;
        assert_eq!(bytes,  file_data.bytes[start.0 ..= start.1]);
    }

    #[test]
    fn test_import() {
        //TESTSAVE0
        let mut file_data = FileData::build("saves/testsave0", PathBuf::from("resources")).unwrap();
        import(&mut file_data, "saves/testexport3").unwrap();

        // Read the exported file into a vector of bytes
        let mut file = File::open("saves/testexport3").unwrap();
        let mut bytes = Vec::new();
        file.read_to_end(&mut bytes).unwrap();

        //Check the contents of the file
        assert_eq!(bytes.len(), APPEARANCE_BYTES_AMOUNT);
        let start = file_data.offsets.appearance;
        assert_eq!(bytes,  file_data.bytes[start.0 ..= start.1]);

        //TESTSAVE1
        let mut file_data = FileData::build("saves/testsave1", PathBuf::from("resources")).unwrap();
        import(&mut file_data, "saves/testexport2").unwrap();

        // Read the exported file into a vector of bytes
        let mut file = File::open("saves/testexport2").unwrap();
        let mut bytes = Vec::new();
        file.read_to_end(&mut bytes).unwrap();

        //Check the contents of the file
        assert_eq!(bytes.len(), APPEARANCE_BYTES_AMOUNT);
        let start = file_data.offsets.appearance;
        assert_eq!(bytes,  file_data.bytes[start.0 ..= start.1]);

        //TESTSAVE2
        let mut file_data = FileData::build("saves/testsave2", PathBuf::from("resources")).unwrap();
        import(&mut file_data, "saves/testexport1").unwrap();

        // Read the exported file into a vector of bytes
        let mut file = File::open("saves/testexport1").unwrap();
        let mut bytes = Vec::new();
        file.read_to_end(&mut bytes).unwrap();

        //Check the contents of the file
        assert_eq!(bytes.len(), APPEARANCE_BYTES_AMOUNT);
        let start = file_data.offsets.appearance;
        assert_eq!(bytes,  file_data.bytes[start.0 ..= start.1]);

        //TESTSAVE3
        let mut file_data = FileData::build("saves/testsave3", PathBuf::from("resources")).unwrap();
        import(&mut file_data, "saves/testexport0").unwrap();

        // Read the exported file into a vector of bytes
        let mut file = File::open("saves/testexport0").unwrap();
        let mut bytes = Vec::new();
        file.read_to_end(&mut bytes).unwrap();

        //Check the contents of the file
        assert_eq!(bytes.len(), APPEARANCE_BYTES_AMOUNT);
        let start = file_data.offsets.appearance;
        assert_eq!(bytes,  file_data.bytes[start.0 ..= start.1]);
    }
}
