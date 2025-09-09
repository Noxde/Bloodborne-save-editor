use super::enums::Error;
use super::file::FileData;
use serde::{Deserialize, Serialize};
use std::f32;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Coordinates {
    offset: usize,
    x: String,
    y: String,
    z: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Pos {
    pub coordinates: Coordinates,
    loaded_map: u32,
}

impl Pos {
    pub fn new(file: &FileData) -> Result<Pos, Error> {
        let bytes = &file.bytes;
        Ok(Pos {
            coordinates: Coordinates::new(file).unwrap(),
            loaded_map: u32::from_le_bytes([bytes[4], bytes[5], bytes[6], bytes[7]]),
        })
    }
}

impl Coordinates {
    pub fn new(file: &FileData) -> Result<Coordinates, Error> {
        let bytes = &file.bytes;
        let lced_offset = file.offsets.lced_offset;

        for i in lced_offset..(bytes.len() - 1) {
            if [
                0xFF, 0xFF, 0xFF, 0xFF, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            ] == bytes[i..=i + 11]
            {
                let x: [u8; 4] = [bytes[i + 12], bytes[i + 13], bytes[i + 14], bytes[i + 15]];
                let y: [u8; 4] = [bytes[i + 16], bytes[i + 17], bytes[i + 18], bytes[i + 19]];
                let z: [u8; 4] = [bytes[i + 20], bytes[i + 21], bytes[i + 22], bytes[i + 23]];

                let x = f32::from_le_bytes(x);
                let y = f32::from_le_bytes(y);
                let z = f32::from_le_bytes(z);

                return Ok(Coordinates {
                    offset: i,
                    x: format!("{:.3}", x),
                    y: format!("{:.3}", y),
                    z: format!("{:.3}", z),
                });
            }
        }
        return Err(Error::CustomError("Coordinates could not be found"));
    }

    pub fn edit(&self, file: &mut FileData, x: f32, y: f32, z: f32) {
        let bytes = &mut file.bytes;
        let coords = [
            f32::to_le_bytes(x),
            f32::to_le_bytes(y),
            f32::to_le_bytes(z),
        ];

        for i in 0..3 {
            bytes[self.offset + 12 + 4 * i..=self.offset + 15 + 4 * i].copy_from_slice(&coords[i]);
        }
    }
}
