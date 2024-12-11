use std::ops::Add;

use super::direction::Direction;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Pos2 {
    pub row: isize,
    pub col: isize,
}

impl Add<Direction> for Pos2 {
    type Output = Pos2;

    fn add(self, direction: Direction) -> Self::Output {
        Self {
            row: self.row + direction.row(),
            col: self.col + direction.col(),
        }
    }
}

impl From<(isize, isize)> for Pos2 {
    fn from((row, col): (isize, isize)) -> Self {
        Self { row, col }
    }
}
