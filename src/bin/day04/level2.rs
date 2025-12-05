use crate::day04::common::Grid;

pub fn run(input: &str) {
    let mut grid = input.parse::<Grid>().expect("Failed to parse grid");
    let mut total_removed = 0;

    loop {
        let previous_paper_rolls = grid.all_paper_roll_positions().count();

        grid = grid.without_removeable_paper_rolls();
        let current_paper_rolls = grid.all_paper_roll_positions().count();
        if current_paper_rolls == previous_paper_rolls {
            break;
        }

        total_removed += previous_paper_rolls - current_paper_rolls;
    }

    println!("Answer is {}", total_removed)
}
