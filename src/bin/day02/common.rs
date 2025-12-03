use std::{ops::RangeInclusive, str::FromStr};

use itertools::Itertools;

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

impl Id {
    pub fn is_invalid(&self) -> bool {
        let string_representation = self.0.to_string();
        let len = string_representation.len();
        if len % 2 != 0 {
            false
        } else {
            let (first, second) = string_representation.split_at(len / 2);
            first == second
        }
    }

    pub fn is_invalid_v2(&self) -> bool {
        let string_representation = self.0.to_string();
        let len = string_representation.len();
        let chunk_counts = 2..=len;
        chunk_counts.into_iter().any(|chunk_count| {
            let chunk_size = len / chunk_count;
            let expected = &string_representation[0..chunk_size];
            let chunks = string_representation.chars().chunks(chunk_size);
            chunks
                .into_iter()
                .all(|chunk| chunk.collect::<String>() == expected)
        })
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

pub struct IdRangeIter {
    range: IdRange,
    current: Id,
}

impl Iterator for IdRangeIter {
    type Item = Id;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current == *self.range.0.end() {
            None
        } else {
            let next = self.current;
            self.current = Id(self.current.0 + 1);
            Some(next)
        }
    }
}

impl IntoIterator for IdRange {
    type IntoIter = IdRangeIter;
    type Item = Id;

    fn into_iter(self) -> Self::IntoIter {
        let current = *self.0.start();

        Self::IntoIter {
            range: self,
            current,
        }
    }
}

impl IdRange {
    pub fn invalid_sum(&self) -> u128 {
        self.clone()
            .into_iter()
            .filter(|id| id.is_invalid())
            .map(|id| id.0)
            .sum()
    }

    pub fn invalid_sum_v2(&self) -> u128 {
        self.clone()
            .into_iter()
            .filter(|id| id.is_invalid_v2())
            .map(|id| id.0)
            .sum()
    }
}
