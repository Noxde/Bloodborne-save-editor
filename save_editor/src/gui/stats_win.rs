use fltk::{button, enums::Align, frame, input, prelude::*};
use fltk_grid::Grid;
use super::main_win::Data;
use std::{cell::RefCell, rc::Rc};

pub fn create() -> Rc<RefCell<Grid>> {
    // Grid
    let mut grid = Grid::new(0, 25, 900, 475, "");
        grid.show_grid(false);
        grid.set_layout(20, 15);
    Rc::new(RefCell::new(grid))
}

pub fn update(data: Rc<RefCell<Data>>, grid_rc: Rc<RefCell<Grid>>) {
    // Grid
    let mut grid = grid_rc.borrow_mut();

    // Clear widgets
    grid.clear();

    // Check data
    if data.borrow().save_data() {
        // Get data
        let mut data_borrow = data.borrow_mut();
        let save_data = data_borrow.data_or_panic();
        
        // Display stats
        let mut lable_current = frame::Frame::default()
            .with_label("Current Value")
            .with_align(Align::Center);
        grid.add(&lable_current);
        grid.set_widget(&mut lable_current, 4, 6);
        
        let mut lable_new = frame::Frame::default()
        .with_label("New Value")
        .with_align(Align::Center);
        grid.add(&lable_new);
        grid.set_widget(&mut lable_new, 4, 8..10);

        let stats_inputs: Rc<RefCell<Vec<input::IntInput>>> = Rc::new(RefCell::new(Vec::new()));
        for (index, stat) in save_data.stats.iter().enumerate() {

            let mut stat_input = input::IntInput::default();
            grid.add(&stat_input);
            grid.set_widget(&mut stat_input, index+5, 8..10);
            stats_inputs.borrow_mut().push(stat_input);

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
            grid.set_widget(&mut stat_value, index + 5, 6);
        }
        
        // Save Changes button
        let mut save_button = button::Button::default().with_label("Save Changes");
        grid.add(&save_button);
        grid.set_widget(&mut save_button, 18..20, 13..15);

        let data_clone = Rc::clone(&data);
        let stats_inputs_clone = Rc::clone(&stats_inputs);
        let grid_clone = Rc::clone(&grid_rc);
        save_button.set_callback( move|_| {
            {
            let mut data_mut_borrow = data_clone.borrow_mut();
            let data_mut = data_mut_borrow.data_or_panic();
            for (index, input) in stats_inputs_clone.borrow_mut().iter().enumerate() {
                if !input.value().is_empty() {
                    let value: u32 = match input.value().parse() {
                        Ok(val) => val,
                        Err(e)  => panic!("Invalid stat value input: {}",e),
                    };
                    data_mut.stats[index].edit(value, &mut data_mut.file);
                }
            }
            }
            // Save the changes to the file
            {
            let temp_clone = Rc::clone(&data_clone);
            let path = &temp_clone.borrow().path.clone();
            match data_clone.borrow_mut().data_or_panic().file.save(&path) {
                Ok(_) => (),
                Err(e) => panic!("Error while tring to save file: {e}"),
            }
            }
            // Show the changes
            update(Rc::clone(&data_clone), Rc::clone(&grid_clone));
        });
    } else {
        // Display message
        let mut lable_no_data = frame::Frame::default()
        .with_label("Please open a file.")
        .with_align(Align::Center);
        grid.add(&lable_no_data);
        grid.set_widget(&mut lable_no_data, 8, 6..9);
    }
}