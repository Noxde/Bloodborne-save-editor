use super::{constants::*, enums::Error};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Offsets {
    pub username: usize,               //Beginning
    pub inventory: (usize, usize),     //Beginning and end
    pub storage: (usize, usize),       //Beginning and end
    pub upgrades: (usize, usize),      //Beginning and end
    pub key_inventory: (usize, usize), //Beginning and end
    pub appearance: (usize, usize),    //Beginning
    pub equipped_gems: (usize, usize), //Beginning and end
    pub lced_offset: usize,
}

impl Offsets {
    //Searches the username and inventories offsets
    pub fn build(bytes: &Vec<u8>) -> Result<Offsets, Error> {
        let mut username_offset = 0;
        let mut inventory_offset = (0, 0);
        let mut upgrades_offset = (START_TO_UPGRADE, 0);
        let mut key_inventory_offset = (0, 0);
        let mut appearance_offset = (0, 0);
        let mut lced_offset = 0;
        let inv_start_bytes = vec![0x40, 0xf0, 0xff, 0xff]; //Bytes the inventory starts with
        let inv_start_bytes_len = inv_start_bytes.len();
        let appearance_start_bytes = [b'F', b'A', b'C', b'E'];
        let lced_bytes = [0x4C, 0x43, 0x45, 0x44];

        let gems = [
            [0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00],
            [0x01, 0x00, 0x00, 0x00, 0x02, 0x00, 0x00, 0x00],
            [0x01, 0x00, 0x00, 0x00, 0x04, 0x00, 0x00, 0x00],
            [0x01, 0x00, 0x00, 0x00, 0x08, 0x00, 0x00, 0x00],
            [0x01, 0x00, 0x00, 0x00, 0x3f, 0x00, 0x00, 0x00],
        ];
        let runes = [
            [0x02, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00],
            [0x02, 0x00, 0x00, 0x00, 0x02, 0x00, 0x00, 0x00],
        ];

        //Get the end offset for the upgrades
        for i in (upgrades_offset.0..(bytes.len())).step_by(40) {
            let current = &bytes[(i + 8)..(i + 16)];

            let is_match =
                runes.iter().any(|&x| current == x) || gems.iter().any(|&x| current == x);

            if !is_match {
                upgrades_offset.1 = i - 1;
                break;
            }
        }
        //Searches for the inv_start_bytes
        for i in 0..(bytes.len() - inv_start_bytes_len) {
            if *inv_start_bytes == bytes[i..(i + inv_start_bytes_len)] {
                //If the beginning of the inventory is found we calculate the username_offset
                //and the beginning of the key_inventory
                inventory_offset.0 = i;
                username_offset = i - USERNAME_TO_INV_OFFSET;
                key_inventory_offset.0 = username_offset + USERNAME_TO_KEY_INV_OFFSET;
                break;
            }
        }

        if inventory_offset.0 == 0 {
            return Err(Error::CustomError("Failed to find username in save data."));
        }

        //Find the end of the inventories
        let end = [0, 0, 0, 0, 0xFF, 0xFF, 0xFF, 0xFF, 0, 0, 0, 0];
        let mut end_offset: Option<usize> = None;
        let mut find_end = |start: usize, allow_empty: bool| -> Result<usize, Error> {
            //Maximum length of the normal inv before it reaches the key inv
            let inv_max_length = USERNAME_TO_KEY_INV_OFFSET - USERNAME_TO_INV_OFFSET;
            let inv_max_length = if bytes.len() < inv_max_length + start {
                bytes.len() - start
            } else {
                inv_max_length
            };
            let mut buffer = [0; 12];
            for i in (start..start + inv_max_length - 15).step_by(16) {
                buffer.copy_from_slice(&bytes[i + 4..=i + 15]);
                if end == buffer {
                    if end_offset.is_none() {
                        if !allow_empty {
                            return Ok(i + 15);
                        }
                        end_offset = Some(i + 15);
                    }
                } else if end_offset.is_some() {
                    end_offset = None;
                }
            }
            match end_offset {
                Some(off) => Ok(off),
                None => Err(Error::CustomError(
                    "Failed to find the end of the inventory.",
                )),
            }
        };

        inventory_offset.1 = username_offset + USERNAME_TO_INV_OFFSET + 1983 * 16; // source for the 1984 slots: https://www.bloodborne-wiki.com/2024/02/full-storage-glitch.html
        key_inventory_offset.1 = find_end(key_inventory_offset.0, false)?;
        let mut last_i: usize = 0;

        //Searches for the appearance_start_bytes
        for i in 0xF000..(bytes.len() - 4) {
            //0xF000: In all the saves i found the save bytes after 0x10000
            if appearance_start_bytes == bytes[i..i + 4] {
                appearance_offset.0 = i + 4;
                appearance_offset.1 = appearance_offset.0 + APPEARANCE_BYTES_AMOUNT - 1;
                last_i = i;
                break;
            }
        }
        if appearance_offset.0 == 0 {
            return Err(Error::CustomError("Failed to find the appearance."));
        }

        // Find lced offset
        for i in last_i..(bytes.len() - 1) {
            if lced_bytes == bytes[i..i + 4] {
                lced_offset = i;
                break;
            }
        }

        let storage_start_offset = inventory_offset.0 + INV_TO_STORAGE_OFFSET;
        let storage_offset = (
            storage_start_offset,
            storage_start_offset + 1984 * 16, // source for the 1984 slots: https://www.bloodborne-wiki.com/2024/02/full-storage-glitch.html
        );

        Ok(Offsets {
            username: username_offset,
            inventory: inventory_offset,
            storage: storage_offset,
            upgrades: upgrades_offset,
            key_inventory: key_inventory_offset,
            appearance: appearance_offset,
            equipped_gems: (0, 0),
            lced_offset,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data_handling::file::FileData;
    use std::path::PathBuf;

    #[test]
    fn offsets_build() {
        //Test with invalid path
        let file_data = FileData::build("invalid", PathBuf::from("resources"));
        assert!(file_data.is_err());
        if let Err(e) = file_data {
            assert!(e.to_string().contains("I/0 error:"));
        }

        //Test with empty save
        let file_data = FileData::build("saves/emptysave", PathBuf::from("resources"));
        assert!(file_data.is_err());
        if let Err(e) = file_data {
            assert_eq!(e.to_string(), "Save error: The selected file is empty.");
        }

        //Test with a save that has no inventory
        let file_data = FileData::build("saves/no_inv_save", PathBuf::from("resources"));
        assert!(file_data.is_err());
        if let Err(e) = file_data {
            assert_eq!(
                e.to_string(),
                "Save error: Failed to find username in save data."
            );
        }

        //Test a save in which the inventory has no end
        let file_data = FileData::build("saves/no_inv_end_save", PathBuf::from("resources"));
        assert!(file_data.is_err());
        if let Err(e) = file_data {
            assert_eq!(
                e.to_string(),
                "Save error: Failed to find the end of the inventory."
            );
        }

        //Test a save with no appearance
        let file_data = FileData::build("saves/noappearancesave0", PathBuf::from("resources"));
        assert!(file_data.is_err());
        if let Err(e) = file_data {
            assert_eq!(e.to_string(), "Save error: Failed to find the appearance.");
        }

        //testsave0
        let file_data = FileData::build("saves/testsave0", PathBuf::from("resources")).unwrap();
        assert_eq!(file_data.offsets.username, 0x8777);
        assert_eq!(file_data.offsets.inventory, (0x894c, 0x8cd0));
        assert_eq!(file_data.offsets.key_inventory, (0x10540, 0x105af));
        assert_eq!(file_data.offsets.upgrades, (84, 163));
        assert_eq!(
            file_data.offsets.appearance,
            (0x10e3c, 0x10e3c + APPEARANCE_BYTES_AMOUNT - 1)
        );

        //testsave1
        let file_data = FileData::build("saves/testsave1", PathBuf::from("resources")).unwrap();
        assert_eq!(file_data.offsets.username, 0xa82b);
        assert_eq!(file_data.offsets.inventory, (0xaa00, 0xb6a4));
        assert_eq!(file_data.offsets.key_inventory, (0x125f4, 0x126e3));
        assert_eq!(file_data.offsets.upgrades, (84, 0x8c3));
        assert_eq!(
            file_data.offsets.appearance,
            (0x12ef0, 0x12ef0 + APPEARANCE_BYTES_AMOUNT - 1)
        );

        //testsave2
        let file_data = FileData::build("saves/testsave2", PathBuf::from("resources")).unwrap();
        assert_eq!(file_data.offsets.username, 0xa86f);
        assert_eq!(file_data.offsets.inventory, (0xaa44, 0xb638));
        assert_eq!(file_data.offsets.key_inventory, (0x12638, 0x12797));
        assert_eq!(file_data.offsets.upgrades, (84, 0x7d3));
        assert_eq!(
            file_data.offsets.appearance,
            (0x12f34, 0x12f34 + APPEARANCE_BYTES_AMOUNT - 1)
        );

        //testsave3
        let file_data = FileData::build("saves/testsave3", PathBuf::from("resources")).unwrap();
        assert_eq!(file_data.offsets.username, 0xb473);
        assert_eq!(file_data.offsets.inventory, (0xb648, 0xc8ac));
        assert_eq!(file_data.offsets.key_inventory, (0x1323c, 0x133db));
        assert_eq!(file_data.offsets.upgrades, (84, 0xf7b));
        assert_eq!(
            file_data.offsets.appearance,
            (0x13b38, 0x13b38 + APPEARANCE_BYTES_AMOUNT - 1)
        );

        //testsave4
        let file_data = FileData::build("saves/testsave4", PathBuf::from("resources")).unwrap();
        assert_eq!(file_data.offsets.username, 0xc85f);
        assert_eq!(file_data.offsets.inventory, (0xca34, 0xe778));
        assert_eq!(file_data.offsets.key_inventory, (0x14628, 0x14857));
        assert_eq!(file_data.offsets.upgrades, (84, 163));
        assert_eq!(
            file_data.offsets.appearance,
            (0x14f24, 0x14f24 + APPEARANCE_BYTES_AMOUNT - 1)
        );

        //testsave8
        let file_data = FileData::build("saves/testsave8", PathBuf::from("resources")).unwrap();
        assert_eq!(file_data.offsets.username, 0x19897);
        assert_eq!(file_data.offsets.inventory, (0x19a6c, 0x210d0));
        assert_eq!(file_data.offsets.key_inventory, (0x21660, 0x218ef));
        assert_eq!(file_data.offsets.upgrades, (84, 0x10ae3));
        assert_eq!(
            file_data.offsets.appearance,
            (0x21f5c, 0x21f5c + APPEARANCE_BYTES_AMOUNT - 1)
        );
    }
}
