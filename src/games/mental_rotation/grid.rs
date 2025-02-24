#[derive(Clone, Copy)]
pub struct GridPos {
    pub x: usize,
    pub y: usize,
}

impl GridPos {
    pub fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }

    pub fn to_pair(&self) -> (usize, usize) {
        (self.x, self.y)
    }
}
