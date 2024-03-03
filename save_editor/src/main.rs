use crate::gui::main_win;
use std::error::Error;

mod data_handling;
mod gui;

fn main() -> Result<(), Box<dyn Error>> {

    main_win::run()?;

    Ok(())
}
