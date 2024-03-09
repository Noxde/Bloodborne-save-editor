use fltk::{button, dialog, enums::Align, frame, input, prelude::*};
use fltk_grid::Grid;
use super::main_win::{Data, center};
use std::{rc::Rc, cell::RefCell};
use crate::data_handling::{save, enums::Error};

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

    // Open file Button
    let mut open_file_button = button::Button::default().with_label("Open File");
    grid.set_widget(&mut open_file_button, 19, 13..15);

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

    let data_clone = Rc::clone(&data);
    open_file_button.set_callback(move|_| {
        let mut data_borrow = data_clone.borrow_mut();
        if data_borrow.usr_and_path() {
            let save_data = save::SaveData::build(
                &data_borrow.path, 
                &data_borrow.username);
            let save_data = match save_data {
                Ok(data) => data,
                Err(Error::IoError(_)) => {
                    dialog::alert(center().0-200, center().1-30, &format!("Failed to open file '{}'",data_borrow.path));
                    return
                },
                Err(Error::CustomError(e)) => {
                    dialog::alert(center().0-200, center().1-30, e);
                    return
                },
                Err(Error::UiError(e)) => panic!("{}",e),
            };
            data_borrow.save_data = Some(save_data);
        } else {
            if data_borrow.username.is_empty() {
                dialog::alert(center().0-200, center().1-30, "Please submit a character name.");
            } else {
                dialog::alert(center().0-200, center().1-30, "Please select a file.");
            }
        }
    });

    grid
}