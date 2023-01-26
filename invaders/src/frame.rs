use crate::{NUM_COLS, NUM_ROWS};

pub type Frame = Vec<Vec<&'static str>>;

// makes new frame, vector of vectors
pub fn new_frame() -> Frame {
    // make columns
    let mut cols = Vec::with_capacity(NUM_COLS);
    for _ in 0..NUM_COLS {
        // make column desirable size
        let mut col = Vec::with_capacity(NUM_ROWS);
        // add space to column entries
        for _ in 0..NUM_ROWS {
            col.push(" ")
        }
        cols.push(col);
    }
    cols
}

// trait to be able to be drawn onto frame
pub trait Drawable {
    fn draw(&self, frame: &mut Frame);
}