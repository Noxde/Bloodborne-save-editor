use crate::gui::gui_main;
use std::error::Error;

mod data_handling;
mod gui;

fn main() -> Result<(), Box<dyn Error>> {

    gui_main::run()?;

    Ok(())
}
