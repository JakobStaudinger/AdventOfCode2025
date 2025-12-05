use std::{
    collections::{HashMap, HashSet},
    str::FromStr,
};

use itertools::Itertools;

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub struct Position {
    x: i32,
    y: i32,
}

#[derive(Clone, Copy)]
pub enum GridCell {
    Empty,
    PaperRoll,
}

pub struct Grid {
    items: HashMap<Position, GridCell>,
}

impl FromStr for GridCell {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "." => Ok(GridCell::Empty),
            "@" => Ok(GridCell::PaperRoll),
            _ => Err("Invalid cell")?,
        }
    }
}

impl FromStr for Grid {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let items = s
            .split('\n')
            .filter(|line| !line.is_empty())
            .enumerate()
            .flat_map(|(row_index, row)| {
                row.chars().enumerate().map(move |(column_index, cell)| {
                    Ok::<_, Self::Err>((
                        Position {
                            x: column_index as i32,
                            y: row_index as i32,
                        },
                        cell.to_string()
                            .parse::<GridCell>()
                            .map_err(|_| "Failed to parse cell")?,
                    ))
                })
            })
            .collect::<Result<HashMap<_, _>, _>>()?;

        Ok(Self { items })
    }
}

impl GridCell {
    pub fn is_paper_roll(&self) -> bool {
        matches!(self, Self::PaperRoll)
    }
}

impl Grid {
    pub fn all_paper_roll_positions(&self) -> impl Iterator<Item = Position> {
        self.items
            .iter()
            .filter(|(_, value)| value.is_paper_roll())
            .map(|(position, _)| *position)
    }

    pub fn neighbors_of(&self, position: &Position) -> impl Iterator<Item = (Position, GridCell)> {
        (-1..=1)
            .cartesian_product(-1..=1)
            .filter(|(x, y)| *x != 0 || *y != 0)
            .flat_map(|(x_offset, y_offset)| {
                self.items
                    .get(&position.offset_by(x_offset, y_offset))
                    .map(|item| (*position, *item))
            })
    }

    pub fn is_paper_roll_removable(&self, position: &Position) -> bool {
        self.neighbors_of(position)
            .filter(|cell| cell.1.is_paper_roll())
            .count()
            < 4
    }

    pub fn without_removeable_paper_rolls(self) -> Self {
        let rolls_to_remove = self
            .all_paper_roll_positions()
            .filter(|position| self.is_paper_roll_removable(position))
            .collect::<HashSet<_>>();

        let mut items = self.items;
        items.retain(|key, _| !rolls_to_remove.contains(key));

        Self { items }
    }
}

impl Position {
    pub fn offset_by(&self, x: i32, y: i32) -> Position {
        Self {
            x: self.x + x,
            y: self.y + y,
        }
    }
}
