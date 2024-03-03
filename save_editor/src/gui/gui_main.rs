use fltk::{enums::Align, app, frame, prelude::*, window::Window};
use fltk_grid::Grid;
use crate::data_handling::{enums, save};

pub fn run() -> Result<(), enums::Error>{

    let data = save::SaveData::build("testsave", "Proyectito")?;

    let app = app::App::default();
    let mut wind = Window::default()
        .with_size(900, 500)
        .center_screen()
        .with_label("Bloodborne save editor.");

    let mut grid = Grid::default_fill();
    grid.show_grid(false); 
    grid.set_layout(20, 15); 

    let mut lable_current = frame::Frame::default().with_label("Current Value").with_align(Align::Center);
    grid.set_widget(&mut lable_current, 4, 5); 
    for (index, stat) in data.player.stats.iter().enumerate() {
        let name_label = format!("{}:",stat.name);
        let mut stat_name = frame::Frame::default().with_label(&name_label).with_align(Align::Right);
        let mut stat_value = frame::Frame::default().with_label(&stat.value.to_string()).with_align(Align::Center);
        grid.set_widget(&mut stat_name, index+5, 3); 
        grid.set_widget(&mut stat_value, index+5, 5); 
    }
    
    wind.end();
    wind.show();


    app.run().map_err(enums::Error::UiError)?;

    Ok(())
}