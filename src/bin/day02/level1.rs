use crate::day02::common::IdRange;

pub fn run(input: &str) {
    let line = input.split('\n').find(|_| true).expect("Missing input");

    let answer: u128 = line
        .split(',')
        .map(|range| range.parse::<IdRange>().expect("Failed to parse range"))
        .map(|range| range.invalid_sum())
        .sum();

    println!("Answer is {}", answer);
}
