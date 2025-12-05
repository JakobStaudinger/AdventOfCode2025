use std::{ops::RangeInclusive, str::FromStr};

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash, Debug)]
pub struct Id(u128);

#[derive(Clone, Hash, PartialEq, Eq, Debug)]
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

impl Id {
    pub fn incremented(self) -> Self {
        Self(self.0 + 1)
    }

    pub fn decremented(self) -> Self {
        Self(self.0 - 1)
    }
}

impl IdRange {
    pub fn contains(&self, id: &Id) -> bool {
        self.0.contains(id)
    }

    pub fn start(&self) -> &Id {
        self.0.start()
    }

    pub fn end(&self) -> &Id {
        self.0.end()
    }

    pub fn number_of_ids(&self) -> usize {
        if self.0.is_empty() {
            0
        } else {
            (self.end().0 - self.start().0 + 1) as usize
        }
    }

    pub fn with_start(self, start: Id) -> Self {
        Self(start..=*self.end())
    }

    pub fn with_end(self, end: Id) -> Self {
        Self(*self.start()..=end)
    }
}
