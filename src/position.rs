

#[derive(Clone)]
pub struct Position {
    from: (usize, usize),
    to: (usize, usize)
}

impl Position {
    pub fn new(line1: usize, col1: usize, line2: usize, col2: usize) -> Position {
        Self {
            from: (line1, col1),
            to: (line2, col2)
        }
    }
    pub fn line(&self) -> usize {
        self.to.0
    }
}


