#![warn(clippy::perf)]

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;

pub use day01::Day01Solver;
pub use day02::Day02Solver;
pub use day03::Day03Solver;
pub use day04::Day04Solver;
pub use day05::Day05Solver;
pub use day06::Day06Solver;

pub trait AocSolver {
    type Output: std::fmt::Display;
    fn part_1(input: &str) -> Self::Output;
    fn part_2(input: &str) -> Self::Output;
}
