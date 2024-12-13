#![warn(clippy::perf)]

pub mod grid;
pub mod mask;
pub mod util;

use paste::paste;

macro_rules! day_modules {
    ($($day:literal),* $(,)?) => {
        paste! {
        $(
            mod [<day $day>];
            pub use [<day $day>]::[<Day $day Solver>];
        )*
        }
    };
}

#[rustfmt::skip]
day_modules![
    01,
    02,
    03,
    04,
    05,
    06,
    07,
    08,
    09,
    10,
    11,
    12,
    13,
];

pub trait AocSolver {
    type Output: std::fmt::Display;
    fn part_1(input: &str) -> Self::Output;
    fn part_2(input: &str) -> Self::Output;
}
