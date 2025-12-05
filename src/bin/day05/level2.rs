use std::collections::HashSet;

use crate::day05::common::{Id, IdRange};

pub fn run(input: &str) {
    let mut iter = input.split("\n\n");
    let ranges = iter.next().expect("Could not find ranges");

    let ranges = ranges
        .split('\n')
        .map(|line| line.parse::<IdRange>())
        .collect::<Result<Vec<_>, _>>()
        .expect("Failed to parse id ranges");

    let non_overlapping_ranges = calculate_non_overlapping_ranges(ranges);

    let answer: usize = non_overlapping_ranges
        .into_iter()
        .map(|range| {
            let num = range.number_of_ids();
            println!("{:?} => {}", range, num);
            num
        })
        .sum();

    println!("Answer is {}", answer);
}

fn calculate_non_overlapping_ranges(ranges: Vec<IdRange>) -> impl Iterator<Item = IdRange> {
    let mut non_overlapping_ranges = HashSet::<IdRange>::new();

    for mut range in ranges {
        let mut ranges_to_remove = HashSet::new();

        for existing_range in &non_overlapping_ranges {
            if existing_range.contains(range.start()) {
                range = range.with_start(existing_range.end().incremented());
            }

            if existing_range.contains(range.end()) {
                range = range.with_end(existing_range.start().decremented())
            }

            if range.contains(existing_range.start()) && range.contains(existing_range.end()) {
                ranges_to_remove.insert(existing_range.clone());
            }
        }

        for range in ranges_to_remove {
            non_overlapping_ranges.remove(&range);
        }

        non_overlapping_ranges.insert(range);
    }

    non_overlapping_ranges.into_iter()
}
