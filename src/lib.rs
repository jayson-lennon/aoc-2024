mod day01;
mod day02;
mod day03;

pub use day01::Day1Solver;
pub use day02::Day2Solver;
pub use day03::Day3Solver;

pub trait AocSolver {
    type Output: std::fmt::Display;
    fn part_1(input: &str) -> Self::Output;
    fn part_2(input: &str) -> Self::Output;
}
