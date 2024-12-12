use std::ops::Index;

pub use direction::Direction;
use fxhash::FxHashSet;
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

pub trait Finder: Clone {
    fn check(&self, ch: char) -> bool;
}

pub trait Query {
    type Output;
    fn query(&mut self, grid: &Grid2D, pos: Pos2) -> Self::Output;
}

pub struct Grid2D {
    inner: Vec<Vec<char>>,
}

impl Grid2D {
    /// Returns the dimensions of the grid.
    #[inline(always)]
    pub fn dim(&self) -> Dimensions2D {
        Dimensions2D {
            rows: self.inner[0].len(),
            cols: self.inner.len(),
        }
    }

    /// Returns the element at `pos`, or `None` if off-grid.
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

    /// Returns `true` if the given position is on the grid.
    #[inline(always)]
    pub fn on_grid<P>(&self, pos: P) -> bool
    where
        P: Into<Pos2>,
    {
        let pos = pos.into();
        let Dimensions2D { rows, cols } = self.dim();

        (pos.row as usize) < rows && (pos.col as usize) < cols
    }

    /// Finds all elements satisfying the `Finder` implementation.
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

    /// Finds all elements satisfying the `Finder` implementation, returning an iterator over the
    /// results.
    pub fn find_all_iter<F>(&self, finder: F) -> impl Iterator<Item = (Pos2, char)> + use<'_, F>
    where
        F: Finder,
    {
        self.inner.iter().enumerate().flat_map(move |(r, row)| {
            row.iter().enumerate().filter_map({
                let finder = finder.clone();
                move |(c, col)| {
                    if finder.check(*col) {
                        let pos = Pos2::from((r as isize, c as isize));
                        Some((pos, *col))
                    } else {
                        None
                    }
                }
            })
        })
    }

    /// Queries the grid at the given position.
    ///
    /// This allows you to run arbitrary code starting from a given position.
    #[inline(always)]
    pub fn query<Q>(&self, mut query: Q, pos: Pos2) -> Q::Output
    where
        Q: Query,
    {
        query.query(self, pos)
    }

    /// Returns all unique elements present in the grid.
    #[inline(always)]
    pub fn unique(&self) -> FxHashSet<char> {
        let mut set = FxHashSet::default();
        for row in &self.inner {
            for col in row {
                set.insert(*col);
            }
        }
        set
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

    #[test]
    fn finds_unique_elements() {
        let grid = Grid2D::from(GRID_3X3);

        let expected = ['0', '1', '2', '3'].into_iter().collect::<FxHashSet<_>>();
        assert_eq!(grid.unique(), expected);
    }
}
