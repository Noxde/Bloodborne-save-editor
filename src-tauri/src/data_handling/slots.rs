use serde::{Deserialize, Serialize};

use super::{enums::{SlotShape, UpgradeType},
           upgrades::Upgrade,
           file::FileData};
use std::collections::HashMap;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Slot {
    pub shape: SlotShape,
    pub gem: Option<Upgrade>,
}

impl Slot {
    fn build(shape: SlotShape, gem: Option<Upgrade>) -> Self {
        Slot {
            shape,
            gem,
        }
    }

}

pub fn parse_equipped_gems(file_data: &mut FileData, upgrades: &mut HashMap<u32, (Upgrade, UpgradeType)>) -> HashMap<u64, Vec<Slot>> {
    let mut slots = HashMap::new();
    let mut abort = true;
    let mut index: usize = 0;
    let mut previous: usize;

    //The block that contains a armor/weapon and its slots is 60B:
    //4B for an unique code.
    //4B for the article id
    //4B for durability
    //4B
    //4B for the start of the slots, it seems to always be 0x01000000
    //Then for each slot (5):
    //4B for the slot shape (or if it is closed)
    //4B for the id of the equipped gem

    //Procedure:
    //First we search for a byte equal to 0x01
    //If it happens to be the start of the slots of a block
    //Then the first block is found, and we can start to process them
    //After some blocks, there is a space filled with 0x00000000ffffffff (garbage)
    //We skip that part and then find the next valid slot block
    //We continue to parse the blocks and the garbage ultil we reach the username offset
    //After which there are no more valid blocks
    //NOTE: Equipped RUNES will also be stored in slots

    //This closure determines if there is a valid block in the received offset
    //If there is, it is added to the HashMap
    //The offset must point to the fist bit of the block
    let mut get_slots = |offset: usize| -> bool {
        let id = u64::from_le_bytes([file_data.bytes[offset+0],
                                     file_data.bytes[offset+1],
                                     file_data.bytes[offset+2],
                                     file_data.bytes[offset+3],
                                     file_data.bytes[offset+4],
                                     file_data.bytes[offset+5],
                                     file_data.bytes[offset+6],
                                     file_data.bytes[offset+7]]);

        if id == 0 {
            return false;
        }

        let mut slots_vec = Vec::with_capacity(5);
        for val in (offset + 20 .. offset + 60).into_iter().step_by(8) {
            let mut gem: Option<Upgrade> = None;
            //If the slot shape is valid
            if let Ok(shape) = SlotShape::try_from(&[file_data.bytes[val+0],
                                                     file_data.bytes[val+1],
                                                     file_data.bytes[val+2],
                                                     file_data.bytes[val+3]]) {

                //If the slot is open we fetch the gem in the slot
                if shape != SlotShape::Closed {

                    let gem_id = u32::from_le_bytes([file_data.bytes[val+4],
                                                 file_data.bytes[val+5],
                                                 file_data.bytes[val+6],
                                                 file_data.bytes[val+7]]);

                    if let Some(upgrade) = upgrades.remove(&gem_id) {
                        gem = Some(upgrade.0);
                    } else {
                        gem = None;
                    }
                }
                slots_vec.push(Slot::build(shape, gem));

            } else {
                return false;
            }

        }

        slots.insert(id, slots_vec);
        //Update the offset of the end of the equipped gems
        //with the end of the last valid slot
        file_data.offsets.equipped_gems.1 = offset + 59;

        true
    };

    //Find the first block
    for i in (file_data.offsets.upgrades.1) .. (file_data.offsets.username - 147) {
        if get_slots(i-16) {
            abort = false;
            index = i - 16 + 60; //Next slots block
            file_data.offsets.equipped_gems.0 = index - 60;
            break;
        }
    }
    if abort {
        return slots;
    }

    //We continue parsing until we reach the first the username data block
    //In this block the first value that appears is the character health (-147)
    //So we use that as ceiling
    while index < (file_data.offsets.username - 147) {
        previous = index;

        //Parse the following blocks
        while get_slots(index) {
            index += 60;
        }

        //Skip the garbage
        while u64::from_le_bytes([file_data.bytes[index+0],
                                   file_data.bytes[index+1],
                                   file_data.bytes[index+2],
                                   file_data.bytes[index+3],
                                   file_data.bytes[index+4],
                                   file_data.bytes[index+5],
                                   file_data.bytes[index+6],
                                   file_data.bytes[index+7]]) == 0xFFFFFFFF00000000 {
            index += 8;
        }

        //If the loop is stuck in corrupted data, continue
        if previous == index {
            index +=1;
        }
    }

    return slots;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data_handling::upgrades::parse_upgrades;
    use std::path::PathBuf;

    #[test]
    fn test_parse_equipped_gems() {
        let mut file_data = FileData::build("saves/testsave9", PathBuf::from("resources")).unwrap();
        let mut upgrades = parse_upgrades(&file_data);
        let slots = parse_equipped_gems(&mut file_data, &mut upgrades);
        assert_eq!(file_data.offsets.equipped_gems, (0x1bc, 0x1bc3));
        //Hunter Axe +3
        let weapon_slots = slots.get(&0x004c4c6c808001d0).unwrap();
        assert_eq!(weapon_slots.len(), 5);

        //Slot 1
        assert_eq!(weapon_slots[0].shape, SlotShape::Radial);
        let gem = weapon_slots[0].gem.clone().unwrap();
        assert_eq!(gem.id, u32::from_le_bytes([0x74, 0x00, 0x80, 0xc0]));
        assert_eq!(gem.shape, String::from("Radial"));
        let info = gem.info.clone();
        assert_eq!(info.name, String::from("Tempering Blood Gemstone (2)"));
        assert_eq!(info.effect, String::from("Physical ATK UP +7.3%"));
        assert_eq!(info.rating, 7);
        assert_eq!(info.level, 2);
        assert_eq!(info.note, String::from(""));

        //Slot 2
        assert_eq!(weapon_slots[1].shape, SlotShape::Radial);
        let gem = weapon_slots[1].gem.clone().unwrap();
        assert_eq!(gem.id, u32::from_le_bytes([0x6f, 0x00, 0x80, 0xc0]));
        assert_eq!(gem.shape, String::from("Radial"));
        let info = gem.info.clone();
        assert_eq!(info.name, String::from("Tempering Blood Gemstone (3)"));
        assert_eq!(info.effect, String::from("Physical ATK UP +9.5%"));
        assert_eq!(info.rating, 9);
        assert_eq!(info.level, 3);
        assert_eq!(info.note, String::from(""));

        //Slot 3
        assert_eq!(weapon_slots[2].shape, SlotShape::Closed);
        assert!(weapon_slots[2].gem.is_none());

        //Slot 4
        assert_eq!(weapon_slots[3].shape, SlotShape::Closed);
        assert!(weapon_slots[3].gem.is_none());

        //Slot 5
        assert_eq!(weapon_slots[4].shape, SlotShape::Closed);
        assert!(weapon_slots[4].gem.is_none());
    }
}
