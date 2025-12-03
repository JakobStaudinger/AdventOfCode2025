use advent_of_code_2025::cli::{self, Level};
use day01::*;

mod day01 {
    mod common;
    pub mod level1;
    pub mod level2;
}

fn main() {
    cli::run("inputs/day01.txt", |level, input| match level {
        Level::Level1 => level1::run(input),
        Level::Level2 => level2::run(input),
    });
}
