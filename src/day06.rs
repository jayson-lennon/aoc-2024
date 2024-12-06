use std::{
    ops::Index,
    sync::atomic::{AtomicU32, Ordering},
};

use rayon::prelude::*;

use crate::AocSolver;

pub struct Day06Solver;

impl AocSolver for Day06Solver {
    type Output = u32;

    fn part_1(input: &str) -> Self::Output {
        let mut map = Map::from(input);
        let guard = map.guard_position().unwrap();
        map.simulate_guard(guard);
        map.visited_count()
    }

    fn part_2(input: &str) -> Self::Output {
        let mut map = Map::from(input);

        let guard = map.guard_position().unwrap();

        // simulate initial guard movement so we know where we can add obstacles
        map.simulate_guard(guard);

        let visited_spaces = map.iter_visited_spaces().collect::<Vec<_>>();

        let possible_loops = AtomicU32::new(0);

        visited_spaces.par_iter().for_each(|pos| {
            let mut map = map.clone();
            map.add_obstruction(pos);
            match map.simulate_guard(guard) {
                SimulationResult::StuckInLoop => {
                    possible_loops.fetch_add(1, Ordering::Relaxed);
                }
                SimulationResult::ExitMap => (),
            }
        });
        possible_loops.load(Ordering::Relaxed)
    }
}

enum SimulationResult {
    ExitMap,
    StuckInLoop,
}

#[derive(Default, Debug, Clone, PartialEq, Eq)]
struct Map {
    inner: Vec<Vec<char>>,
}

impl Map {
    #[inline(always)]
    fn has_obstacle(&self, pos: Pos) -> bool {
        is_obstacle(self.inner[pos.row()][pos.col()])
    }

    fn guard_position(&self) -> Option<Guard> {
        self.inner.iter().enumerate().find_map(|(i, col)| {
            col.iter()
                .enumerate()
                .find_map(|(j, &ch)| {
                    if ch == '^' || ch == 'v' || ch == '<' || ch == '>' {
                        Some((j, ch))
                    } else {
                        None
                    }
                })
                .map(|(j, ch)| Guard {
                    pos: (i, j).into(),
                    facing: ch.into(),
                })
        })
    }

    #[inline(always)]
    fn bounds(&self) -> (usize, usize) {
        (self.inner.len(), self.inner[0].len())
    }

    #[inline(always)]
    fn set_visited(&mut self, pos: Pos) {
        self.inner[pos.row()][pos.col()] = 'X';
    }

    fn simulate_guard(&mut self, mut guard: Guard) -> SimulationResult {
        let mut loop_counter = 0;
        loop {
            match guard.next(self) {
                GuardMovement::Straight { old, .. } => {
                    self.set_visited(old);
                }
                GuardMovement::Turned { .. } => {}
                GuardMovement::OffMap { old } => {
                    self.set_visited(old);
                    break SimulationResult::ExitMap;
                }
            }
            loop_counter += 1;
            if loop_counter > 6500 {
                break SimulationResult::StuckInLoop;
            }
        }
    }

    #[inline(always)]
    fn add_obstruction(&mut self, pos: &Pos) {
        self.inner[pos.row()][pos.col()] = '#';
    }

    fn iter_visited_spaces(&self) -> impl Iterator<Item = Pos> + '_ {
        self.inner.iter().enumerate().flat_map(|(i, row)| {
            row.iter()
                .enumerate()
                .filter_map(|(j, &ch)| if ch == 'X' { Some((i, j).into()) } else { None })
                .collect::<Vec<_>>()
        })
    }

    fn visited_count(&self) -> u32 {
        self.inner
            .iter()
            .map(|row| row.iter().filter(|&&ch| ch == 'X').count() as u32)
            .sum()
    }
}

#[inline(always)]
fn is_obstacle(entry: char) -> bool {
    entry == '#'
}

impl Index<(usize, usize)> for Map {
    type Output = char;

    fn index(&self, (row, col): (usize, usize)) -> &Self::Output {
        &self.inner[row][col]
    }
}

/// Grid parser
impl From<&str> for Map {
    fn from(value: &str) -> Self {
        Self {
            inner: value.lines().map(|line| line.chars().collect()).collect(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl From<char> for Direction {
    fn from(ch: char) -> Self {
        match ch {
            '^' => Direction::Up,
            'v' => Direction::Down,
            '>' => Direction::Right,
            '<' => Direction::Left,
            _ => unimplemented!(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Pos {
    row: usize,
    col: usize,
}

impl Pos {
    fn row(&self) -> usize {
        self.row
    }
    fn col(&self) -> usize {
        self.col
    }
}

impl From<(usize, usize)> for Pos {
    #[inline(always)]
    fn from((row, col): (usize, usize)) -> Self {
        Pos { row, col }
    }
}

// Row/Col coordinate for the grid
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Guard {
    pos: Pos,
    facing: Direction,
}

enum GuardMovement {
    Straight { old: Pos },
    Turned,
    OffMap { old: Pos },
}

impl Guard {
    #[inline(always)]
    fn next(&mut self, map: &Map) -> GuardMovement {
        match self.facing {
            Direction::Up => self.move_up(map),
            Direction::Down => self.move_down(map),
            Direction::Left => self.move_left(map),
            Direction::Right => self.move_right(map),
        }
    }

    #[inline(always)]
    fn move_up(&mut self, map: &Map) -> GuardMovement {
        let old = self.pos;
        if self.pos.row > 0 {
            let new = (self.pos.row - 1, self.pos.col).into();
            if map.has_obstacle(new) {
                self.facing = Direction::Right;
                GuardMovement::Turned
            } else {
                self.pos.row -= 1;
                GuardMovement::Straight { old }
            }
        } else {
            GuardMovement::OffMap { old }
        }
    }

    #[inline(always)]
    fn move_down(&mut self, map: &Map) -> GuardMovement {
        let old = self.pos;
        if self.pos.row < map.bounds().0 - 1 {
            let new = (self.pos.row + 1, self.pos.col).into();
            if map.has_obstacle(new) {
                self.facing = Direction::Left;
                GuardMovement::Turned
            } else {
                self.pos.row += 1;
                GuardMovement::Straight { old }
            }
        } else {
            GuardMovement::OffMap { old }
        }
    }

    #[inline(always)]
    fn move_left(&mut self, map: &Map) -> GuardMovement {
        let old = self.pos;
        if self.pos.col > 0 {
            let new = (self.pos.row, self.pos.col - 1).into();
            if map.has_obstacle(new) {
                self.facing = Direction::Up;
                GuardMovement::Turned
            } else {
                self.pos.col -= 1;
                GuardMovement::Straight { old }
            }
        } else {
            GuardMovement::OffMap { old }
        }
    }

    #[inline(always)]
    fn move_right(&mut self, map: &Map) -> GuardMovement {
        let old = self.pos;
        if self.pos.col < map.bounds().1 - 1 {
            let new = (self.pos.row, self.pos.col + 1).into();
            if map.has_obstacle(new) {
                self.facing = Direction::Down;
                GuardMovement::Turned
            } else {
                self.pos.col += 1;
                GuardMovement::Straight { old }
            }
        } else {
            GuardMovement::OffMap { old }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = r#"....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...
"#;

    const SAMPLE_PATH: &str = r#"....#.....
....XXXXX#
....X...X.
..#.X...X.
..XXXXX#X.
..X.X.X.X.
.#XXXXXXX.
.XXXXXXX#.
#XXXXXXX..
......#X.."#;

    const SAMPLE_OBSTRUCTION: &str = r#"....#.....
....+---+#
....|...|.
..#.|...|.
..+-+-+#|.
..|.|.|.|.
.#+-^-+-+.
....|.|.#.
#..O+-+...
......#..."#;

    #[test]
    fn parses_into_grid() {
        let grid = Map::from(SAMPLE);
        assert_eq!(grid[(0, 4)], '#');
        assert_eq!(grid[(6, 4)], '^');
    }

    #[test]
    fn gets_guard_position() {
        let grid = Map::from(SAMPLE);
        assert_eq!(
            grid.guard_position(),
            Some(Guard {
                pos: Pos { row: 6, col: 4 },
                facing: Direction::Up
            })
        );
    }

    #[test]
    fn simulates_guard_movement() {
        let mut map = Map::from(SAMPLE);
        let guard = map.guard_position().unwrap();
        map.simulate_guard(guard);

        assert_eq!(map, Map::from(SAMPLE_PATH));
    }

    #[test]
    fn solves_part_1() {
        let answer = Day06Solver::part_1(SAMPLE);
        assert_eq!(answer, 41);
    }

    #[test]
    fn solves_part_2() {
        let answer = Day06Solver::part_2(SAMPLE_OBSTRUCTION);
        assert_eq!(answer, 6);
    }
}
