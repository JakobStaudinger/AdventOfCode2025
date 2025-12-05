use crate::day05::common::{Id, IdRange};

pub fn run(input: &str) {
    let mut iter = input.split("\n\n");
    let ranges = iter.next().expect("Could not find ranges");
    let ingredients = iter.next().expect("Could not find ingredients");

    let ranges = ranges
        .split('\n')
        .map(|line| line.parse::<IdRange>())
        .collect::<Result<Vec<_>, _>>()
        .expect("Failed to parse id ranges");

    let ingredients = ingredients
        .split('\n')
        .filter(|line| !line.is_empty())
        .map(|line| line.parse::<Id>())
        .collect::<Result<Vec<_>, _>>()
        .expect("Failed to parse ingredients");

    let answer = ingredients
        .into_iter()
        .filter(|ingredient| ranges.iter().any(|range| range.contains(&ingredient)))
        .count();

    println!("Answer is {}", answer);
}
