use advent_of_code_2025::cli::{self, Level};
use day02::*;

mod day02 {
    mod common;
    pub mod level1;
    pub mod level2;
}

fn main() {
    cli::run("inputs/day02.txt", |level, input| match level {
        Level::Level1 => level1::run(input),
        Level::Level2 => level2::run(input),
    });
}
