use fxhash::{FxHashMap, FxHashSet};
use smallvec::SmallVec;
use std::collections::hash_map;

use crate::AocSolver;

pub struct Day08Solver;

impl AocSolver for Day08Solver {
    type Output = u32;

    fn part_1(input: &str) -> Self::Output {
        let map = FrequencyMap::from(input);

        map.iter_antennas()
            .flat_map(|(_, antennas)| {
                let mut antinodes: SmallVec<[Pos; 12]> = SmallVec::default();
                // check each antenna against all other antennas
                for a in antennas {
                    for b in antennas {
                        if a != b {
                            let antinode = a.antinode(b);
                            if antinode.on_map(&map) {
                                antinodes.push(antinode);
                            }
                        }
                    }
                }
                assert!(!antinodes.spilled());
                antinodes
            })
            .collect::<FxHashSet<_>>()
            .len() as u32
    }

    fn part_2(input: &str) -> Self::Output {
        let map = FrequencyMap::from(input);

        map.iter_antennas()
            .flat_map(|(_, antennas)| {
                let mut antinodes = Vec::with_capacity(112);
                // check each antenna against all other antennas
                for a in antennas {
                    for b in antennas {
                        if a != b {
                            // translate antinodes from a to b
                            let dist = a.distance(b);
                            let mut antinode = a.translate(dist);

                            while antinode.on_map(&map) {
                                antinodes.push(antinode);
                                antinode = antinode.translate(dist);
                            }

                            // translate antinodes from b to a
                            let diff = b.distance(a);
                            antinode = b.translate(diff);

                            while antinode.on_map(&map) {
                                antinodes.push(antinode);
                                antinode = antinode.translate(diff);
                            }
                        }
                    }
                }
                antinodes
            })
            .collect::<FxHashSet<_>>()
            .len() as u32
    }
}

#[derive(Debug)]
struct FrequencyMap {
    antennas: FxHashMap<char, Vec<Pos>>,
    width: i8,
    height: i8,
}

impl FrequencyMap {
    pub fn iter_antennas(&self) -> hash_map::Iter<'_, char, Vec<Pos>> {
        self.antennas.iter()
    }
}

impl From<&str> for FrequencyMap {
    fn from(value: &str) -> Self {
        let mut antennas = FxHashMap::default();
        let (mut width, mut height) = (0, 0);
        for (row, line) in value.lines().enumerate() {
            for (col, ch) in line.chars().enumerate() {
                if ch != '.' {
                    let entry: &mut Vec<Pos> = antennas.entry(ch).or_default();
                    entry.push((row as i8, col as i8).into());
                }
                width = col;
            }
            height = row;
        }

        Self {
            antennas,
            width: width as i8 + 1,
            height: height as i8 + 1,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Pos {
    row: i8,
    col: i8,
}

impl Pos {
    #[inline(always)]
    fn row(&self) -> i8 {
        self.row
    }

    #[inline(always)]
    fn col(&self) -> i8 {
        self.col
    }

    /// Translates this position based on a translation distance.
    #[inline(always)]
    fn translate(&self, (row, col): (i8, i8)) -> Self {
        Pos {
            row: self.row + row,
            col: self.col + col,
        }
    }

    /// Returns the position of an antinode.
    #[inline(always)]
    fn antinode(&self, other: &Pos) -> Pos {
        Pos {
            row: self.row() - (other.row() - self.row()),
            col: self.col() - (other.col() - self.col()),
        }
    }

    /// Returns the distance between this position and another.
    #[inline(always)]
    fn distance(&self, other: &Pos) -> (i8, i8) {
        (other.row() - self.row(), other.col() - self.col())
    }

    /// Returns `true` if this position is on the map.
    #[inline(always)]
    fn on_map(&self, map: &FrequencyMap) -> bool {
        let (row, col) = (self.row, self.col);
        row >= 0 && row < map.height && col >= 0 && col < map.width
    }
}

impl From<(i8, i8)> for Pos {
    #[inline(always)]
    fn from((row, col): (i8, i8)) -> Self {
        Pos { row, col }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = r#"............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............"#;

    const SAMPLE_2: &str = r#"T.........
...T......
.T........
..........
..........
..........
..........
..........
..........
.........."#;

    #[test]
    fn parses() {
        let map = FrequencyMap::from(SAMPLE);
        assert_eq!(map.antennas[&'0'][0], Pos::from((1, 8)));
    }

    #[test]
    fn calculates_antinode() {
        let this_pos = Pos::from((2, 5));

        let other_pos = Pos::from((3, 8));

        assert_eq!(this_pos.antinode(&other_pos), Pos::from((1, 2)));
    }

    #[test]
    fn solve_part_1() {
        let answer = Day08Solver::part_1(SAMPLE);

        assert_eq!(answer, 14);
    }

    #[test]
    fn solve_part_2() {
        let answer = Day08Solver::part_2(SAMPLE_2);

        assert_eq!(answer, 9);
    }
}
