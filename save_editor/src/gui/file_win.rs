use fltk::{button, dialog, enums::Align, frame, input, prelude::*};
use fltk_grid::Grid;
use super::main_win::Data;
use std::{rc::Rc, cell::RefCell};

pub fn display(data: Rc<RefCell<Data>>) -> Grid {
    // Grid
    let mut grid = Grid::new(0, 25, 900, 475, "");
        grid.show_grid(false);
        grid.set_layout(20, 15);
    
    // Username input
    let mut lable_message = frame::Frame::default()
        .with_label("Enter your character name:")
        .with_align(Align::Center);
    grid.set_widget(&mut lable_message, 7, 6..9);
    
    let mut username_input = input::Input::default();
    grid.set_widget(&mut username_input, 8, 6..9);

    // Buttons
    // Select file Button
    let mut select_file_button = button::Button::default().with_label("Select File");
    grid.set_widget(&mut select_file_button, 19, 11..13);

    // Submit username Button
    let mut submit_button = button::Button::default().with_label("Submit");
    grid.set_widget(&mut submit_button, 10, 6..9);

    // Callbacks
    let data_clone = Rc::clone(&data);
    select_file_button.set_callback(move|_| {
        let mut dialog = dialog::NativeFileChooser::new(dialog::NativeFileChooserType::BrowseFile);
        dialog.show();
        let filename = format!("{:?}", dialog.filename());
        data_clone.borrow_mut().path = filename.trim_matches('"').to_string();
    });

    let data_clone = Rc::clone(&data);
    submit_button.set_callback(move|_| {
        let username = username_input.value();
        data_clone.borrow_mut().username = username;
    });

    grid
}