use super::file::FileData;

pub struct PlayerData {
    pub health: u32,
    pub stamina: u32,

    pub echoes: u32,
    pub insight: u32,

    pub level: u32,
    pub vitality: u32,
    pub endurance: u32,
    pub strength: u32,
    pub skill: u32,
    pub bloodtinge: u32,
    pub arcane: u32,
}

impl PlayerData {
    pub fn new(file: &FileData) -> PlayerData {
        // HP, Stamina
        let health = file.get_number(-147, 4);
        let stamina = file.get_number(-119, 4);
        // Currencies
        let echoes = file.get_number(-19, 4);
        let insight = file.get_number(-35, 4);
        // Stats
        let level = file.get_number(-23, 4);
        let vitality = file.get_number(-103, 4);
        let endurance = file.get_number(-95, 4);
        let strength = file.get_number(-79, 4);
        let skill = file.get_number(-71, 4);
        let bloodtinge = file.get_number(-63, 4);
        let arcane = file.get_number(-55, 4);

        PlayerData {
            health,
            stamina,

            echoes,
            insight,

            level,
            vitality,
            endurance,
            strength,
            skill,
            bloodtinge,
            arcane,
        }
    }
}
