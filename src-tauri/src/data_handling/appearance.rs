use super::{file::FileData,
            enums::Error};
use std::{fs::{self, File},
          io::Read};

pub fn export(file_data: &FileData, path: &str) -> Result<(), Error> {
    let mut appearance_end = 0;

    //Find the end of the appearance
    let end = [0, 0, 0, 0, 0xFF, 0xFF, 0xFF, 0xFF, 0, 0, 0, 0, 0, 0, 0, 0];
    let mut buffer = [0; 16];
    for i in (file_data.offsets.appearance .. file_data.bytes.len() - 15).step_by(16) {
            buffer.copy_from_slice(&file_data.bytes[i ..= i + 15]);
            if end == buffer {
                appearance_end = i - 1;
                break;
            }
        }
    if appearance_end == 0 {
       return Err(Error::CustomError("Failed to find the end of the appearance."));
    }

    let mut export_bytes = Vec::new();
    export_bytes.extend_from_slice(&file_data.bytes[file_data.offsets.appearance ..= appearance_end]);

    fs::write(path, &export_bytes).map_err(Error::IoError)
}

pub fn import(file_data: &mut FileData, path: & str) -> Result<(), Error> {
    // Read the exported file into a vector of bytes
    let mut file = File::open(path).map_err(Error::IoError)?;
    let mut bytes = Vec::new();
    file.read_to_end(&mut bytes).map_err(Error::IoError)?;
    let length = bytes.len();
    let start = file_data.offsets.appearance;
    for i in start .. start + length {
        file_data.bytes[i] = bytes[i - start];
    }

    //If the new appearance is bigger than the previous one the difference must be cleaned
    //Find the end of the appearance
    let end = [0, 0, 0, 0, 0xFF, 0xFF, 0xFF, 0xFF, 0, 0, 0, 0, 0, 0, 0, 0];
    let mut buffer = [0; 16];
    for i in (start + length .. file_data.bytes.len() - 15).step_by(16) {
        buffer.copy_from_slice(&file_data.bytes[i ..= i + 15]);
        if end == buffer {
            break;
        } else {
            file_data.bytes.splice(i .. i + 16, end);
        }
    }

    Ok(())
}


#[cfg(test)]
mod tests {
    use super::*;
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
        assert_eq!(bytes.len(),0x130);
        let start = file_data.offsets.appearance;
        let end = [0, 0, 0, 0, 0xFF, 0xFF, 0xFF, 0xFF, 0, 0, 0, 0, 0, 0, 0, 0];
        assert_eq!(bytes,  file_data.bytes[start .. start + 0x130]);
        assert_eq!(end, file_data.bytes[start + 0x130 .. start + 0x130 + 16]);

        //TESTSAVE1
        let file_data = FileData::build("saves/testsave1", PathBuf::from("resources")).unwrap();
        export(&file_data, "saves/testexport1").unwrap();

        // Read the exported file into a vector of bytes
        let mut file = File::open("saves/testexport1").unwrap();
        let mut bytes = Vec::new();
        file.read_to_end(&mut bytes).unwrap();

        //Check the contents of the file
        assert_eq!(bytes.len(),0x220);
        let start = file_data.offsets.appearance;
        let end = [0, 0, 0, 0, 0xFF, 0xFF, 0xFF, 0xFF, 0, 0, 0, 0, 0, 0, 0, 0];
        assert_eq!(bytes,  file_data.bytes[start .. start + 0x220]);
        assert_eq!(end, file_data.bytes[start + 0x220 .. start + 0x220 + 16]);

        //TESTSAVE2
        let file_data = FileData::build("saves/testsave2", PathBuf::from("resources")).unwrap();
        export(&file_data, "saves/testexport2").unwrap();

        // Read the exported file into a vector of bytes
        let mut file = File::open("saves/testexport2").unwrap();
        let mut bytes = Vec::new();
        file.read_to_end(&mut bytes).unwrap();

        //Check the contents of the file
        assert_eq!(bytes.len(),0x190);
        let start = file_data.offsets.appearance;
        let end = [0, 0, 0, 0, 0xFF, 0xFF, 0xFF, 0xFF, 0, 0, 0, 0, 0, 0, 0, 0];
        assert_eq!(bytes,  file_data.bytes[start .. start + 0x190]);
        assert_eq!(end, file_data.bytes[start + 0x190 .. start + 0x190 + 16]);

        //TESTSAVE3
        let file_data = FileData::build("saves/testsave3", PathBuf::from("resources")).unwrap();
        export(&file_data, "saves/testexport3").unwrap();

        // Read the exported file into a vector of bytes
        let mut file = File::open("saves/testexport3").unwrap();
        let mut bytes = Vec::new();
        file.read_to_end(&mut bytes).unwrap();

        //Check the contents of the file
        assert_eq!(bytes.len(),0x310);
        let start = file_data.offsets.appearance;
        let end = [0, 0, 0, 0, 0xFF, 0xFF, 0xFF, 0xFF, 0, 0, 0, 0, 0, 0, 0, 0];
        assert_eq!(bytes,  file_data.bytes[start .. start + 0x310]);
        assert_eq!(end, file_data.bytes[start + 0x310 .. start + 0x310 + 16]);
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
        assert_eq!(bytes.len(),0x310);
        let start = file_data.offsets.appearance;
        let end = [0, 0, 0, 0, 0xFF, 0xFF, 0xFF, 0xFF, 0, 0, 0, 0, 0, 0, 0, 0];
        assert_eq!(bytes,  file_data.bytes[start .. start + 0x310]);
        assert_eq!(end, file_data.bytes[start + 0x310 .. start + 0x310 + 16]);

        //TESTSAVE1
        let mut file_data = FileData::build("saves/testsave1", PathBuf::from("resources")).unwrap();
        import(&mut file_data, "saves/testexport2").unwrap();

        // Read the exported file into a vector of bytes
        let mut file = File::open("saves/testexport2").unwrap();
        let mut bytes = Vec::new();
        file.read_to_end(&mut bytes).unwrap();

        //Check the contents of the file
        assert_eq!(bytes.len(),0x190);
        let start = file_data.offsets.appearance;
        let end = [0, 0, 0, 0, 0xFF, 0xFF, 0xFF, 0xFF, 0, 0, 0, 0, 0, 0, 0, 0];
        assert_eq!(bytes,  file_data.bytes[start .. start + 0x190]);
        assert_eq!(end, file_data.bytes[start + 0x190 .. start + 0x190 + 16]);

        //TESTSAVE2
        let mut file_data = FileData::build("saves/testsave2", PathBuf::from("resources")).unwrap();
        import(&mut file_data, "saves/testexport1").unwrap();

        // Read the exported file into a vector of bytes
        let mut file = File::open("saves/testexport1").unwrap();
        let mut bytes = Vec::new();
        file.read_to_end(&mut bytes).unwrap();

        //Check the contents of the file
        assert_eq!(bytes.len(),0x220);
        let start = file_data.offsets.appearance;
        let end = [0, 0, 0, 0, 0xFF, 0xFF, 0xFF, 0xFF, 0, 0, 0, 0, 0, 0, 0, 0];
        assert_eq!(bytes,  file_data.bytes[start .. start + 0x220]);
        assert_eq!(end, file_data.bytes[start + 0x220 .. start + 0x220 + 16]);

        //TESTSAVE3
        let mut file_data = FileData::build("saves/testsave3", PathBuf::from("resources")).unwrap();
        import(&mut file_data, "saves/testexport0").unwrap();

        // Read the exported file into a vector of bytes
        let mut file = File::open("saves/testexport0").unwrap();
        let mut bytes = Vec::new();
        file.read_to_end(&mut bytes).unwrap();

        //Check the contents of the file
        assert_eq!(bytes.len(),0x130);
        let start = file_data.offsets.appearance;
        let end = [0, 0, 0, 0, 0xFF, 0xFF, 0xFF, 0xFF, 0, 0, 0, 0, 0, 0, 0, 0];
        assert_eq!(bytes,  file_data.bytes[start .. start + 0x130]);
        assert_eq!(end, file_data.bytes[start + 0x130 .. start + 0x130 + 16]);
    }
}
