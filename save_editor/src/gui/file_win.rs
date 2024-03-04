use fltk::{enums::Align, frame, prelude::*};
use fltk_grid::Grid;

pub fn display() -> Grid {
    
    // Grid
    let mut grid = Grid::new(0, 25, 900, 475, "");
        grid.show_grid(false);
        grid.set_layout(20, 15);
    
    // Display message
    let mut lable_message = frame::Frame::default()
        .with_label("This is the 'File' window.")
        .with_align(Align::Center);
    grid.set_widget(&mut lable_message, 10, 7);

    grid
}