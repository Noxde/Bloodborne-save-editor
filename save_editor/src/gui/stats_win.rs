use fltk::{enums::Align, frame, prelude::*};
use fltk_grid::Grid;
use super::main_win::Data;
use std::{rc::Rc, cell::RefCell};

pub fn create() -> Grid {
    // Grid
    let mut grid = Grid::new(0, 25, 900, 475, "");
        grid.show_grid(false);
        grid.set_layout(20, 15);
    grid
}

pub fn update(data: Rc<RefCell<Data>>, grid: &mut Grid) {
    // Clear widgets
    grid.clear();

    // Check data
    if data.borrow().save_data() {
        // Get data
        let data_borrow = data.borrow();
        let save_data = data_borrow.data_or_panic();
        
        // Display stats
        let mut lable_current = frame::Frame::default()
            .with_label("Current Value")
            .with_align(Align::Center);
        grid.add(&lable_current);
        grid.set_widget(&mut lable_current, 4, 5);
        for (index, stat) in save_data.stats.iter().enumerate() {
            let name_label = format!("{}:", stat.name);

            let mut stat_name = frame::Frame::default()
                .with_label(&name_label)
                .with_align(Align::Right);
            let mut stat_value = frame::Frame::default()
                .with_label(&stat.value.to_string())
                .with_align(Align::Center);

            grid.add(&stat_name);
            grid.set_widget(&mut stat_name, index + 5, 3);
            grid.add(&stat_value);
            grid.set_widget(&mut stat_value, index + 5, 5);
        }
    } else {
        // Display message
        let mut lable_current = frame::Frame::default()
        .with_label("Please open a file.")
        .with_align(Align::Center);
        grid.add(&lable_current);
        grid.set_widget(&mut lable_current, 8, 6..9);
    }
}