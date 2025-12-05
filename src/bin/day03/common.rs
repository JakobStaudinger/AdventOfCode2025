use std::str::FromStr;

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct Joltage(u8);

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct Battery(Joltage);

pub struct BatteryBank {
    batteries: Vec<Battery>,
}

impl FromStr for Joltage {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Joltage(s.parse().map_err(|_| "Could not parse integer")?))
    }
}

impl FromStr for Battery {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 1 {
            Err("Battery Joltage must be exactly one digit")?
        }

        Ok(Battery(s.parse().map_err(|_| "Could not parse joltage")?))
    }
}

impl FromStr for BatteryBank {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let batteries = s
            .chars()
            .map(|part| part.to_string().parse::<Battery>())
            .collect::<Result<_, _>>()?;

        Ok(BatteryBank { batteries })
    }
}

impl Joltage {
    pub fn value(&self) -> u8 {
        self.0
    }
}

impl BatteryBank {
    pub fn largest_possible_joltage(&self) -> Joltage {
        let result = self
            .batteries
            .iter()
            .rfold((None, None), |(first, second), item| {
                match (first, second) {
                    (None, _) => (Some(item), None),
                    (Some(first), None) => (Some(item), Some(first)),
                    (Some(first), Some(second)) => {
                        if item >= first {
                            (Some(item), Some(first.max(second)))
                        } else {
                            (Some(first), Some(second))
                        }
                    }
                }
            });

        if let (Some(largest), Some(second_largest)) = result {
            Joltage(largest.0.0 * 10 + second_largest.0.0)
        } else {
            panic!()
        }
    }
}
