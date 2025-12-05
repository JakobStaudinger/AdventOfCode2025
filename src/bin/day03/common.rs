use std::str::FromStr;

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct Joltage(u64);

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
    pub fn value(&self) -> u64 {
        self.0
    }
}

impl BatteryBank {
    pub fn largest_possible_joltage(&self, num_batteries: usize) -> Joltage {
        let chosen_batteries =
            self.batteries
                .iter()
                .rfold(Vec::new(), |mut chosen_batteries, battery| {
                    if chosen_batteries.len() < num_batteries {
                        chosen_batteries.insert(0, battery);
                    } else if battery >= chosen_batteries.first().unwrap() {
                        let mut index_to_drop = 0;
                        for (i, battery) in chosen_batteries.iter().enumerate() {
                            if *battery > chosen_batteries[index_to_drop] {
                                break;
                            }

                            index_to_drop = i
                        }

                        chosen_batteries.remove(index_to_drop);
                        chosen_batteries.insert(0, battery);
                    }

                    chosen_batteries
                });

        let value = chosen_batteries
            .into_iter()
            .enumerate()
            .map(|(index, item)| 10_u64.pow((num_batteries - index - 1) as u32) * (item.0.0 as u64))
            .sum();

        Joltage(value)
    }
}
