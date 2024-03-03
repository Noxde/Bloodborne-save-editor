use crate::data_handling::save::SaveData;
use std::error::Error;

mod data_handling;

fn main() -> Result<(), Box<dyn Error>> {
    let data = SaveData::build("testsave", "Proyectito")?;
    for s in data.player.stats.iter() {
        println!("{}: {}", s.name, s.value);
    }

    Ok(())
}
