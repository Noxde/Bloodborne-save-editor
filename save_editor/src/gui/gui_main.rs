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
    grid.show_grid(true); 
    grid.set_layout(20, 15); 

    let mut health_name = frame::Frame::default().with_label("Health:").with_align(Align::Right);
    let mut health_value = frame::Frame::default().with_label(&data.player.health.to_string()).with_align(Align::Center);

    let mut stamina_name = frame::Frame::default().with_label("Stamina:").with_align(Align::Right);
    let mut stamina_value = frame::Frame::default().with_label(&data.player.stamina.to_string()).with_align(Align::Center);

    let mut echoes_name = frame::Frame::default().with_label("Echoes:").with_align(Align::Right);
    let mut echoes_value = frame::Frame::default().with_label(&data.player.echoes.to_string()).with_align(Align::Center);

    let mut insight_name = frame::Frame::default().with_label("Insight:").with_align(Align::Right);
    let mut insight_value = frame::Frame::default().with_label(&data.player.insight.to_string()).with_align(Align::Center);

    let mut level_name = frame::Frame::default().with_label("Level:").with_align(Align::Right);
    let mut level_value = frame::Frame::default().with_label(&data.player.level.to_string()).with_align(Align::Center);

    let mut vitality_name = frame::Frame::default().with_label("Vitality:").with_align(Align::Right);
    let mut vitality_value = frame::Frame::default().with_label(&data.player.vitality.to_string()).with_align(Align::Center);

    let mut endurance_name = frame::Frame::default().with_label("Endurance:").with_align(Align::Right);
    let mut endurance_value = frame::Frame::default().with_label(&data.player.endurance.to_string()).with_align(Align::Center);

    let mut strength_name = frame::Frame::default().with_label("Strength:").with_align(Align::Right);
    let mut strength_value = frame::Frame::default().with_label(&data.player.strength.to_string()).with_align(Align::Center);

    let mut skill_name = frame::Frame::default().with_label("Skill:").with_align(Align::Right);
    let mut skill_value = frame::Frame::default().with_label(&data.player.skill.to_string()).with_align(Align::Center);

    let mut bloodtinge_name = frame::Frame::default().with_label("Bloodtinge:").with_align(Align::Right);
    let mut bloodtinge_value = frame::Frame::default().with_label(&data.player.bloodtinge.to_string()).with_align(Align::Center);

    let mut arcane_name = frame::Frame::default().with_label("Arcane:").with_align(Align::Right);
    let mut arcane_value = frame::Frame::default().with_label(&data.player.arcane.to_string()).with_align(Align::Center);


    grid.set_widget(&mut health_name, 2, 2); 
    grid.set_widget(&mut health_value, 2, 4); 
    grid.set_widget(&mut stamina_name, 3, 2); 
    grid.set_widget(&mut stamina_value, 3, 4); 
    grid.set_widget(&mut echoes_name, 4, 2); 
    grid.set_widget(&mut echoes_value, 4, 4); 
    grid.set_widget(&mut insight_name, 5, 2); 
    grid.set_widget(&mut insight_value, 5, 4); 
    grid.set_widget(&mut level_name, 6, 2); 
    grid.set_widget(&mut level_value, 6, 4); 
    grid.set_widget(&mut vitality_name,7, 2); 
    grid.set_widget(&mut vitality_value, 7, 4); 
    grid.set_widget(&mut endurance_name, 8, 2); 
    grid.set_widget(&mut endurance_value, 8, 4); 
    grid.set_widget(&mut strength_name, 9, 2); 
    grid.set_widget(&mut strength_value, 9, 4); 
    grid.set_widget(&mut skill_name, 10, 2); 
    grid.set_widget(&mut skill_value, 10, 4); 
    grid.set_widget(&mut bloodtinge_name, 11, 2); 
    grid.set_widget(&mut bloodtinge_value, 11, 4); 
    grid.set_widget(&mut arcane_name, 12, 2); 
    grid.set_widget(&mut arcane_value, 12, 4); 

    
    wind.end();
    wind.show();


    app.run().map_err(enums::Error::UiError)?;

    Ok(())
}