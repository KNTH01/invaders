use crate::{NUM_COLS, NUM_ROWS};

pub type Frame = Vec<Vec<&'static str>>;

pub fn new_frame() -> Frame {
    let mut rows = Vec::with_capacity(NUM_ROWS);

    for _ in 0..NUM_ROWS {
        let mut cols = Vec::with_capacity(NUM_COLS);
        for _ in 0..NUM_COLS {
            cols.push(" ")
        }
        rows.push(cols)
    }
    
    rows
}

pub trait Drawable {
    fn draw(&self, frame: &mut Frame);
}
