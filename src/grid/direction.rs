#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Direction {
    row: isize,
    col: isize,
}

impl Direction {
    pub fn row(&self) -> isize {
        self.row
    }

    pub fn col(&self) -> isize {
        self.col
    }
}

impl From<(isize, isize)> for Direction {
    fn from((row, col): (isize, isize)) -> Self {
        Direction { row, col }
    }
}
