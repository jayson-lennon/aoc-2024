mod day01;
mod day02;
mod day03;
mod day04;
mod day05;

pub use day01::Day1Solver;
pub use day02::Day2Solver;
pub use day03::Day3Solver;
pub use day04::Day4Solver;
pub use day05::Day5Solver;

pub trait AocSolver {
    type Output: std::fmt::Display;
    fn part_1(input: &str) -> Self::Output;
    fn part_2(input: &str) -> Self::Output;
}
