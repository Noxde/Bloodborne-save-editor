use crate::data_handling::save::SaveData;
use std::error::Error;

mod data_handling;
mod gui;

fn main() -> Result<(), Box<dyn Error>> {
    let data = SaveData::build("testsave", "Proyectito")?;
    println!("Health: {}", data.player.health);
    println!("Stamina: {}", data.player.stamina);
    println!("Level: {}", data.player.level);
    println!("Blood echoes: {}", data.player.echoes);
    println!("Insight: {}", data.player.insight);

    Ok(())
}
