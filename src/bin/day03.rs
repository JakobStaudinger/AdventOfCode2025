use advent_of_code_2025::cli::{self, Level};
use day03::*;

mod day03 {
    mod common;
    pub mod level1;
    pub mod level2;
}

fn main() {
    cli::run("inputs/day03.txt", |level, input| match level {
        Level::Level1 => level1::run(input),
        Level::Level2 => level2::run(input),
    });
}
