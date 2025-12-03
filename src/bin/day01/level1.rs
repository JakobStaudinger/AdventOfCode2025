use crate::day01::common::Dial;

pub fn run(input: &str) {
    let sum = input
        .split('\n')
        .filter(|line| !line.is_empty())
        .map(|line| line.parse().expect("Malformed input"))
        .scan(Dial::new(), |state, value| {
            *state = state.rotated_by(&value);
            Some(state.is_zero())
        })
        .filter(|value| *value)
        .count();

    println!("Answer is {sum}");
}
