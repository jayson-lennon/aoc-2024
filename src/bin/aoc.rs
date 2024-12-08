use aoc_2024::AocSolver;
use clap::{command, Parser};
use color_eyre::eyre::Result;
use paste::paste;
use std::path::PathBuf;
use tap::Pipe;

use mimalloc::MiMalloc;
#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

macro_rules! pick_solver {
    ($select:expr, $data:expr, $($day:literal),* $(,)?) => {
        paste! {
        match $select {
            $(
                $day => {
                    run::<[<Day $day Solver>], _>(&$data)?.pipe(print_solution);
                }
            )*
        _ => eprintln!("solution not found"),
        }
        }
    };
}

/// Advent of Code 2024 runner
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Which day to run
    day: u8,

    /// Path to data file, if outside of data dir
    #[arg(short, long)]
    data_file: Option<PathBuf>,
}

fn main() -> Result<()> {
    color_eyre::install()?;

    let args = Args::parse();

    let data = {
        let path = if let Some(path) = args.data_file {
            path
        } else {
            PathBuf::from(format!("data/day{:02}.txt", args.day))
        };
        std::fs::read_to_string(path)?
    };

    start(args.day, data)
}

#[allow(clippy::zero_prefixed_literal)]
fn start(day: u8, data: String) -> Result<()> {
    use aoc_2024::*;

    pick_solver!(day, data, 01, 02, 03, 04, 05, 06, 07, 08);

    Ok(())
}

fn run<S, T>(data: &str) -> Result<(T, T)>
where
    S: AocSolver<Output = T>,
    T: std::fmt::Display,
{
    Ok((S::part_1(data), S::part_2(data)))
}

fn print_solution<T>((part1, part2): (T, T))
where
    T: std::fmt::Display,
{
    println!("Part 1: {part1}");
    println!("Part 2: {part2}");
}
