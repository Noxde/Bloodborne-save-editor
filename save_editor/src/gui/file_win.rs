use fltk::{enums::Align, frame, prelude::*};
use fltk_grid::Grid;
use crate::data_handling::save;

pub fn display(data: save::SaveData) -> Grid {
    
    //Main grid
    let mut grid = Grid::default_fill();
        grid.show_grid(false);
        grid.set_layout(20, 15);
    
    // Display stats
    let mut lable_current = frame::Frame::default()
        .with_label("Current Value")
        .with_align(Align::Center);
    grid.set_widget(&mut lable_current, 4, 5);
    for (index, stat) in data.stats.iter().enumerate() {
        let name_label = format!("{}:", stat.name);

        let mut stat_name = frame::Frame::default()
            .with_label(&name_label)
            .with_align(Align::Right);
        let mut stat_value = frame::Frame::default()
            .with_label(&stat.value.to_string())
            .with_align(Align::Center);

        grid.set_widget(&mut stat_name, index + 5, 3);
        grid.set_widget(&mut stat_value, index + 5, 5);
    }
    
    grid
}