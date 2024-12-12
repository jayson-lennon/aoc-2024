use aoc_2024::*;
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use paste::paste;
use std::path::{Path, PathBuf};

use mimalloc::MiMalloc;
#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

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

macro_rules! aoc_bench {
    ($($day:literal),* $(,)?) => {
    paste! {
        $(
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
        )*
        criterion_group!(benches,
            $(
                [<day $day>],
            )*
        );
    }
    };
}

#[rustfmt::skip]
aoc_bench![
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
];

// criterion_group!(benches, day01, day02, day03, day04, day05, day06);
criterion_main!(benches);
