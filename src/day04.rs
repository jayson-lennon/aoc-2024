use std::ops::Index;

use crate::AocSolver;

pub struct Day4Solver;

impl AocSolver for Day4Solver {
    type Output = u32;

    // Strategy: Iterate over each character until we find an `X` or `S`. Once found, perform a
    // search in all directions for `XMAS`.
    fn part_1(input: &str) -> Self::Output {
        let grid = Grid::from(input);

        let mut match_count = 0;
        for (ch, idx) in grid.iter() {
            if ch == 'X' || ch == 'S' {
                for direction in DIRECTIONS {
                    if let ['X', 'M', 'A', 'S'] = grid.find_sequence(4, *direction, idx).as_slice()
                    {
                        match_count += 1;
                    }
                }
            }
        }
        match_count
    }

    #[rustfmt::skip]
    // Strategy: Iterate over each character until we find an `M` or `S`. Once found, copy out a
    // block from the grid large enough to contain an X-MAS and then match on all possible
    // permutations.
    fn part_2(input: &str) -> Self::Output {
        let grid = Grid::from(input);
        let mut match_count = 0;
        for (ch, idx) in grid.iter() {
            if ch == 'M' || ch == 'S' {
                match grid
                    .get_block(idx)
                    .as_slice()
                {
                    [
                        'M', _ ,'S',
                         _ ,'A', _,
                        'M', _, 'S',
                    ] | [
                        'S', _, 'S',
                         _, 'A', _,
                        'M', _, 'M',
                    ] | [
                        'S', _, 'M',
                         _, 'A', _,
                        'S', _, 'M',
                    ] | [
                        'M', _, 'M',
                         _, 'A', _,
                        'S', _, 'S',
                    ] => match_count += 1,
                    _ => (),
                }
            }
        }
        match_count
    }
}

/// Search directions
const DIRECTIONS: &[(isize, isize)] = &[
    (1, 0),   // N
    (-1, 0),  // S
    (0, 1),   // E
    (0, -1),  // W
    (1, 1),   // NE
    (1, -1),  // NW
    (-1, 1),  // SE
    (-1, -1), // SW
];

// Row/Col coordinate for the grid
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct GridIndex {
    row: usize,
    col: usize,
}

impl GridIndex {
    fn row(&self) -> usize {
        self.row
    }
    fn col(&self) -> usize {
        self.col
    }
}

impl From<(usize, usize)> for GridIndex {
    fn from((row, col): (usize, usize)) -> Self {
        GridIndex { row, col }
    }
}

#[derive(Default, Debug, Clone)]
struct Grid {
    inner: Vec<Vec<char>>,
}

impl Grid {
    fn rows(&self) -> usize {
        self.inner.len()
    }
    fn cols(&self) -> usize {
        self.inner[0].len()
    }

    /// Returns a sequence of characters up to `n_chars` in length at the given index.
    fn find_sequence<D, I>(&self, n_chars: usize, direction: D, at: I) -> Vec<char>
    where
        D: Into<Direction>,
        I: Into<GridIndex>,
    {
        let direction = direction.into();
        let mut at = at.into();

        // returned character sequence
        let mut sequence = Vec::default();

        // start with the character at the provided index
        sequence.push(self.inner[at.row()][at.col()]);

        for _ in 0..n_chars - 1 {
            let next_row = {
                let next = at.row() as isize + direction.row_mod();
                // whenever we go < 0 that means we are off the grid, return the current sequence
                if next < 0 {
                    return sequence;
                } else {
                    next as usize
                }
            };
            let next_col = {
                let next = at.col() as isize + direction.col_mod();
                // whenever we go < 0 that means we are off the grid, return the current sequence
                if next < 0 {
                    return sequence;
                } else {
                    next as usize
                }
            };

            // whenever we hit the length of a row or col, then we are at the edge of the grid.
            // return the current sequence
            if next_row == self.rows() || next_col == self.cols() {
                break;
            }

            sequence.push(self.inner[next_row][next_col]);

            // update the current index
            at = GridIndex {
                row: next_row,
                col: next_col,
            };
        }

        sequence
    }

    /// Iterator over all characters in the grid
    fn iter(&self) -> impl Iterator<Item = (char, GridIndex)> + '_ {
        self.inner.iter().enumerate().flat_map(move |(i, row)| {
            row.iter()
                .enumerate()
                .map(move |(j, &ch)| (ch, (i, j).into()))
        })
    }

    /// Returns a 2d box of characters sized for the X-MAS in part 2
    fn get_block<I>(&self, at: I) -> Vec<char>
    where
        I: Into<GridIndex>,
    {
        let at = at.into();
        if at.row() > self.rows().saturating_sub(3) {
            return Vec::default();
        }
        if at.col() > self.cols().saturating_sub(3) {
            return Vec::default();
        }

        let mut block = Vec::default();
        for i in 0..=2 {
            for j in 0..=2 {
                block.push(self.inner[at.row() + i][at.col() + j]);
            }
        }
        block
    }
}

impl Index<GridIndex> for Grid {
    type Output = char;

    fn index(&self, GridIndex { row, col }: GridIndex) -> &Self::Output {
        &self.inner[row][col]
    }
}

impl Index<(usize, usize)> for Grid {
    type Output = char;

    fn index(&self, (row, col): (usize, usize)) -> &Self::Output {
        &self.inner[row][col]
    }
}

/// Grid parser
impl From<&str> for Grid {
    fn from(value: &str) -> Self {
        Self {
            inner: value.lines().map(|line| line.chars().collect()).collect(),
        }
    }
}

type Row = isize;
type Col = isize;

/// Direction to search.
///
/// Values should be 0, 1, or -1.
///
/// - 1 goes up(row) or right(col)
/// - 0 doesn't change
/// - -1 goes down(row) or left(col)
struct Direction(Row, Col);

impl Direction {
    fn row_mod(&self) -> isize {
        self.0
    }
    fn col_mod(&self) -> isize {
        self.1
    }
}

impl From<(isize, isize)> for Direction {
    fn from((row, col): (isize, isize)) -> Self {
        Direction(row, col)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = r#"MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX"#;

    #[test]
    fn parses_into_2d_grid() {
        let grid = Grid::from(SAMPLE);

        assert_eq!(grid.rows(), 10);
        assert_eq!(grid.cols(), 10);
    }

    #[test]
    fn grid_indexed_access() {
        let grid = Grid::from(SAMPLE);

        assert_eq!(grid[(0, 0)], 'M');
        assert_eq!(grid[(9, 9)], 'X');
    }

    #[test]
    fn finds_sequence_given_a_direction() {
        let grid = Grid::from(SAMPLE);

        let sequence = grid.find_sequence(4, Direction(0, 1), (0, 4));
        assert_eq!(sequence, &['X', 'X', 'M', 'A']);
    }

    #[test]
    fn finds_sequence_given_a_negative_direction() {
        let grid = Grid::from(SAMPLE);

        let sequence = grid.find_sequence(4, Direction(-1, 0), (2, 0));
        assert_eq!(sequence, &['A', 'M', 'M']);
    }

    #[test]
    fn find_sequence_at_edge_of_grid() {
        let grid = Grid::from(SAMPLE);

        let sequence = grid.find_sequence(4, Direction(0, 1), (0, 8));
        assert_eq!(sequence, &['S', 'M']);
    }

    #[test]
    fn grid_iters_through_chars() {
        let grid = Grid::from(SAMPLE);

        let mut xmas_iter = grid.iter();

        assert_eq!(
            xmas_iter.next().unwrap(),
            ('M', GridIndex { row: 0, col: 0 })
        );
        assert_eq!(
            xmas_iter.next().unwrap(),
            ('M', GridIndex { row: 0, col: 1 })
        );
        assert_eq!(
            xmas_iter.next().unwrap(),
            ('M', GridIndex { row: 0, col: 2 })
        );
        assert_eq!(
            xmas_iter.next().unwrap(),
            ('S', GridIndex { row: 0, col: 3 })
        );
    }

    #[test]
    #[rustfmt::skip]
    fn copy_block_out_of_grid() {
        let grid = Grid::from(SAMPLE);

        let block = grid.get_block((0, 1));

        assert_eq!(block,
            &['M', 'M', 'S',
              'S', 'A', 'M',
              'M', 'X', 'S']);
    }

    #[test]
    fn returns_empty_block_when_it_goes_off_the_grid() {
        let grid = Grid::from(SAMPLE);

        let expected: Vec<char> = Vec::default();

        let block = grid.get_block((0, 8));
        assert_eq!(block, expected);

        let block = grid.get_block((8, 0));
        assert_eq!(block, expected);
    }

    #[test]
    fn solve_part_1() {
        let answer = Day4Solver::part_1(SAMPLE);
        assert_eq!(answer, 18);
    }

    #[test]
    fn solve_part_2() {
        let answer = Day4Solver::part_2(SAMPLE);
        assert_eq!(answer, 9);
    }
}
