use crate::data_handling::{enums, save};
use fltk::{app, prelude::*, window::Window};
use super::file_win;

pub fn run() -> Result<(), enums::Error> {
    let data = save::SaveData::build("testsave", "Proyectito")?;

    let app = app::App::default();
    let mut wind = Window::default()
        .with_size(900, 500)
        .center_screen()
        .with_label("Bloodborne save editor.");

    let mut _file_grid = file_win::display(data);

    wind.end();
    wind.show();

    app.run().map_err(enums::Error::UiError)?;

    Ok(())
}
