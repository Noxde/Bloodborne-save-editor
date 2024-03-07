use crate::data_handling::{enums, save};
use fltk::{app, prelude::*, window::Window, button};
use fltk_grid::Grid;
use super::{stats_win, file_win};
use std::{cell::RefCell, rc::Rc};


pub fn run() -> Result<(), enums::Error> {

    let data = Rc::new(RefCell::new(Data::new()));
    
    // Main App
    let app = app::App::default();
    let mut wind = Window::default()
    .with_size(900, 500)
    .center_screen()
    .with_label("Bloodborne save editor.");

    //Main grid
    let mut main_grid = Grid::new(0, 0, 900, 25, "");
    main_grid.show_grid(false);
    main_grid.set_layout(1, 15);

    // Buttons
    // File button
    let mut file_button = button::RadioButton::default().with_label("File");
    main_grid.set_widget(&mut file_button, 0, 0);

    // Stats button
    let mut stats_button = button::RadioButton::default().with_label("Stats");
    main_grid.set_widget(&mut stats_button, 0, 1);

    main_grid.end();

    // Windows grids
    let stats_grid = stats_win::create();
    stats_grid.borrow_mut().end();
    let file_grid = Rc::new(RefCell::new(file_win::display(Rc::clone(&data))));
    file_grid.borrow_mut().end();

    let file_grid_clone1 = Rc::clone(&file_grid);
    let stats_grid_clone1 = Rc::clone(&stats_grid);
    file_button.set_callback(move |_| {
        stats_grid_clone1.borrow_mut().hide();
        file_grid_clone1.borrow_mut().show();
    });

    let file_grid_clone2 = Rc::clone(&file_grid);
    let stats_grid_clone2 = Rc::clone(&stats_grid);
    stats_button.set_callback(move |_| {
        stats_win::update(data.clone(), Rc::clone(&stats_grid_clone2));
        file_grid_clone2.borrow_mut().hide();
        stats_grid_clone2.borrow_mut().show();
    });

    wind.end();
    wind.show();

    app.run().map_err(enums::Error::UiError)?;

    Ok(())
}

pub struct Data {
    pub save_data: Option<save::SaveData>,
    pub username: String,
    pub path: String,
}

impl Data {
    pub fn new() -> Data {
        Data {
            save_data: None,
            username: String::from(""),
            path: String::from(""),
        }
    }
    
    pub fn usr_and_path(&self) -> bool {
        !self.username.is_empty() && !self.path.is_empty()   
    }

    pub fn save_data(&self) -> bool {
        match self.save_data {
            Some(_) => true,
            None => false,
        }
    }

    pub fn data_or_panic(&mut self) -> &mut save::SaveData {
        match &mut self.save_data {
            Some(data) => data,
            None => panic!("save_data field of Data struct was None"),
        }
    }
}