pub mod component;
pub mod frame;

#[derive(Debug)]
pub struct Layout {
    x: usize,
    y: usize,
    w: usize,
    h: usize,
}

impl Layout {
    pub fn new(x: usize, y: usize, w: usize, h: usize) -> Self {
        Self { x, y, w, h }
    }

    pub fn get_possition(&self) -> (usize, usize) {
        (self.x, self.y)
    }

    pub fn get_size(&self) -> (usize, usize) {
        (self.w, self.h)
    }
}
