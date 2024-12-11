use std::ops::Index;

pub use direction::Direction;
pub use position::Pos2;

mod direction;
mod position;

pub struct Dimensions2D {
    rows: usize,
    cols: usize,
}

impl Dimensions2D {
    pub fn rows(&self) -> usize {
        self.rows
    }

    pub fn cols(&self) -> usize {
        self.cols
    }
}

pub trait Finder {
    fn check(&self, ch: char) -> bool;
}

pub trait Query {
    type Output;
    fn query(&self, grid: &Grid2D, pos: Pos2) -> Self::Output;
}

pub struct Grid2D {
    inner: Vec<Vec<char>>,
}

impl Grid2D {
    #[inline(always)]
    pub fn dim(&self) -> Dimensions2D {
        Dimensions2D {
            rows: self.inner[0].len(),
            cols: self.inner.len(),
        }
    }

    #[inline(always)]
    pub fn get<P>(&self, pos: P) -> Option<char>
    where
        P: Into<Pos2>,
    {
        let pos = pos.into();
        if self.on_grid(pos) {
            Some(self[pos])
        } else {
            None
        }
    }

    #[inline(always)]
    pub fn on_grid<P>(&self, pos: P) -> bool
    where
        P: Into<Pos2>,
    {
        let pos = pos.into();
        let Dimensions2D { rows, cols } = self.dim();

        (pos.row as usize) < rows && (pos.col as usize) < cols
    }

    pub fn find_all<F>(&self, finder: F) -> Vec<(Pos2, char)>
    where
        F: Finder,
    {
        let mut found = Vec::default();
        for (r, row) in self.inner.iter().enumerate() {
            for (c, col) in row.iter().enumerate() {
                if finder.check(*col) {
                    let pos = Pos2::from((r as isize, c as isize));
                    found.push((pos, *col));
                }
            }
        }
        found
    }

    #[inline(always)]
    pub fn query<Q>(&self, query: Q, pos: Pos2) -> Q::Output
    where
        Q: Query,
    {
        query.query(self, pos)
    }
}

impl Index<Pos2> for Grid2D {
    type Output = char;

    #[inline(always)]
    fn index(&self, pos: Pos2) -> &Self::Output {
        &self.inner[pos.row as usize][pos.col as usize]
    }
}

impl From<&str> for Grid2D {
    fn from(value: &str) -> Self {
        Grid2D {
            inner: value
                .lines()
                .map(|row| row.chars().collect::<Vec<_>>())
                .collect(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const GRID_3X3: &str = r#"010
020
030"#;

    #[test]
    fn makes_2d_grid_from_str() {
        let grid = Grid2D::from(GRID_3X3);

        assert_eq!(grid.dim().rows(), 3);
        assert_eq!(grid.dim().cols(), 3);
    }

    #[test]
    fn determines_if_coord_is_on_map() {
        let grid = Grid2D::from(GRID_3X3);

        assert!(grid.on_grid((0, 0)));
        assert!(grid.on_grid((1, 1)));
        assert!(!grid.on_grid((3, 0)));
        assert!(!grid.on_grid((0, 3)));
    }
}
