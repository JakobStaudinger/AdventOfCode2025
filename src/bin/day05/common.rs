use std::{ops::RangeInclusive, str::FromStr};

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct Id(u128);

#[derive(Clone)]
pub struct IdRange(RangeInclusive<Id>);

impl FromStr for Id {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Id(s.parse().map_err(|_| "Failed to parse id")?))
    }
}

impl FromStr for IdRange {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<_> = s.split('-').collect();
        if parts.len() != 2 {
            Err("Unexpected number of ids")?
        }

        let min = parts[0].parse()?;
        let max = parts[1].parse()?;

        Ok(IdRange(min..=max))
    }
}

impl IdRange {
    pub fn contains(&self, id: &Id) -> bool {
        self.0.contains(id)
    }
}
