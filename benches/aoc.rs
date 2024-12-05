use paste::paste;
use std::path::{Path, PathBuf};

use aoc_2024::{AocSolver, Day01Solver, Day02Solver, Day03Solver, Day04Solver, Day05Solver};
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn load_data_file<P: AsRef<Path>>(path: P) -> String {
    std::fs::read_to_string(path).unwrap()
}

// manuall impl
// fn day1(c: &mut Criterion) {
//     let day1 = load_data_file(PathBuf::from("data/day01.txt"));
//     c.bench_function("day01 part 1", |b| {
//         b.iter(|| Day01Solver::part_1(black_box(&day1)))
//     });
//     c.bench_function("day01 part 2", |b| {
//         b.iter(|| Day01Solver::part_2(black_box(&day1)))
//     });
// }

macro_rules! aoc {
    ($($day:literal),* $(,)?) => {
        $(
            paste! {
            fn [<day $day>](c: &mut Criterion) {
                let day_data =
                    load_data_file(PathBuf::from(concat!("data/day", stringify!($day), ".txt")));
                c.bench_function(concat!("day", stringify!($day), " part 1"), |b| {
                    b.iter(|| [<Day $day Solver>]::part_1(black_box(&day_data)))
                });
                c.bench_function(concat!("day", stringify!($day), " part 2"), |b| {
                    b.iter(|| [<Day $day Solver>]::part_2(black_box(&day_data)))
                });
            }
            }
        )*
    };
}

aoc![01, 02, 03, 04, 05];

criterion_group!(benches, day01, day02, day03, day04, day05);
criterion_main!(benches);
