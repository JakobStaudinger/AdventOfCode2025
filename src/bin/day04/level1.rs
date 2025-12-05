use crate::day04::common::Grid;

pub fn run(input: &str) {
    let grid = input.parse::<Grid>().expect("Failed to parse grid");

    let accessible_paper_rolls = grid
        .all_paper_roll_positions()
        .filter(|position| grid.is_paper_roll_removable(position))
        .count();

    println!("Answer is {}", accessible_paper_rolls);
}
