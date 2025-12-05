use crate::day03::common::BatteryBank;

pub fn run(input: &str) {
    let banks = input
        .split('\n')
        .filter(|line| !line.is_empty())
        .map(|line| line.parse::<BatteryBank>())
        .collect::<Result<Vec<_>, _>>()
        .expect("Failed to parse battery banks");

    let result: u32 = banks
        .into_iter()
        .map(|bank| bank.largest_possible_joltage())
        .map(|joltage| joltage.value() as u32)
        .sum();

    println!("Answer is {}", result);
}
