pub struct PlayerData {
    pub health: u32,
}

impl PlayerData {
    pub fn new(save_data: &Vec<u8>, username_offset: usize) -> PlayerData {
        let health_offset = username_offset-147;
        let health_bytes = &save_data[health_offset..health_offset+2];

        let mut health: u32 = 0;
        let base: u32 = 256;

        for (index, byte) in health_bytes.iter().enumerate().rev() {
            health += *byte as u32 * (base.pow(index as u32));
        }

        PlayerData { health, }
    }
}