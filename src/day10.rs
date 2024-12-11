use crate::{
    grid::{Direction, Finder, Grid2D, Pos2, Query},
    AocSolver,
};
use fxhash::FxHashSet;
use itertools::Itertools;
use smallvec::SmallVec;

pub struct Day10Solver;

impl AocSolver for Day10Solver {
    type Output = u32;

    fn part_1(input: &str) -> Self::Output {
        let grid = Grid2D::from(input);

        let trailheads = grid.find_all(Zeroes);

        let scores = explore(TrailheadRule::UniqueSummits, &grid, &trailheads);
        scores.iter().unique().map(|(_, score)| score).sum::<u32>()
    }

    fn part_2(input: &str) -> Self::Output {
        let grid = Grid2D::from(input);

        let trailheads = grid.find_all(Zeroes);

        let scores = explore(TrailheadRule::DistinctTrails, &grid, &trailheads);

        scores.iter().unique().map(|(_, score)| score).sum::<u32>()
    }
}

/// Whether we want to get unique summits (part 1) or distinct trails (part 2)
#[derive(Debug, Clone, Copy)]
enum TrailheadRule {
    UniqueSummits,
    DistinctTrails,
}

/// Grid finder to locate all positions containing a '0'.
struct Zeroes;

impl Finder for Zeroes {
    #[inline(always)]
    fn check(&self, ch: char) -> bool {
        ch == '0'
    }
}

/// Grid query to find all adjacent positions having a number 1 greater than the query position.
struct FindAdjacentNumbersLargerByOne;

impl Query for FindAdjacentNumbersLargerByOne {
    type Output = SmallVec<[Pos2; 4]>;

    #[inline(always)]
    fn query(&self, grid: &Grid2D, pos: Pos2) -> Self::Output {
        let mut adjacent = SmallVec::default();
        let current = grid[pos] as u8;

        // up, down, left, right
        for direction in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
            let check_pos = pos + Direction::from(direction);
            if let Some(next) = grid.get(check_pos) {
                if (next as u8) == current + 1 {
                    adjacent.push(check_pos);
                }
            }
        }

        adjacent
    }
}

/// Recursive exploration entry point
fn explore(rule: TrailheadRule, grid: &Grid2D, trailheads: &[(Pos2, char)]) -> Vec<(Pos2, u32)> {
    // Need to track the trailheads that we have already visited for part1
    let mut visited = FxHashSet::default();

    trailheads
        .iter()
        .map(|(pos, _)| explore_impl(rule, grid, &mut visited, *pos, *pos))
        .collect()
}

/// Recursively find trails
fn explore_impl(
    rule: TrailheadRule,
    grid: &Grid2D,
    visited: &mut FxHashSet<(Pos2, Pos2)>,
    trailhead: Pos2,
    current: Pos2,
) -> (Pos2, u32) {
    let adjacent = grid.query(FindAdjacentNumbersLargerByOne, current);

    if adjacent.is_empty() {
        // reached the end of a trail
        match rule {
            TrailheadRule::UniqueSummits => {
                if grid[current] == '9' && !visited.contains(&(trailhead, current)) {
                    visited.insert((trailhead, current));
                    return (trailhead, 1);
                } else {
                    return (trailhead, 0);
                }
            }
            TrailheadRule::DistinctTrails => {
                if grid[current] == '9' {
                    return (trailhead, 1);
                } else {
                    return (trailhead, 0);
                }
            }
        }
    }

    let score = adjacent
        .iter()
        .map(|pos| explore_impl(rule, grid, visited, trailhead, *pos).1)
        .sum::<u32>();

    (trailhead, score)
}

#[cfg(test)]
mod tests {
    use crate::AocSolver;

    use super::Day10Solver;

    const TEST_GRID: &str = r#"89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732"#;

    #[test]
    fn solve_part_1() {
        let answer = Day10Solver::part_1(TEST_GRID);
        assert_eq!(answer, 36);
    }

    #[test]
    fn solve_part_2() {
        let answer = Day10Solver::part_2(TEST_GRID);
        assert_eq!(answer, 81);
    }
}
