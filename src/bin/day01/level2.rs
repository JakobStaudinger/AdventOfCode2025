use crate::day01::common::{Dial, Rotation};

pub fn run(input: &str) {
    let sum = input
        .split('\n')
        .filter(|line| !line.is_empty())
        .map(|line| line.parse::<Rotation>().expect("Malformed input"))
        .flat_map(|value| value.as_clicks())
        .scan(Dial::new(), |state, value| {
            *state = state.rotated_by(&value);
            Some(state.is_zero())
        })
        .filter(|value| *value)
        .count();

    println!("Answer is {sum}");
}
