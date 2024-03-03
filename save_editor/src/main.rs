use std::error::Error;
use gui::gui_main;

mod data_handling;
mod gui;

fn main() -> Result<(), Box<dyn Error>> {

    gui_main::run()?;

    Ok(())
}
