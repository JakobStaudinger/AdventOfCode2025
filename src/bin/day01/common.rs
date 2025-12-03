use std::str::FromStr;

pub enum Rotation {
    Left(i32),
    Right(i32),
}

impl FromStr for Rotation {
    type Err = String;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        if line.len() == 0 {
            return Err("Empty line".to_string());
        }

        let val = line[1..]
            .parse()
            .map_err(|_| "Could not parse integer".to_string())?;

        match &line[0..1] {
            "L" => Ok(Rotation::Left(val)),
            "R" => Ok(Rotation::Right(val)),
            _ => Err("Unknown direction".to_string()),
        }
    }
}

impl Rotation {
    pub fn as_clicks(self) -> impl Iterator<Item = Rotation> {
        let to_click = |amount| {
            if amount < 0 {
                Rotation::Left(1)
            } else {
                Rotation::Right(1)
            }
        };

        match self {
            Rotation::Left(amount) => ((-amount)..0).map(to_click),
            Rotation::Right(amount) => (1..(amount + 1)).map(to_click),
        }
    }
}

pub struct Dial {
    state: i32,
}

impl Dial {
    pub fn new() -> Self {
        Dial { state: 50 }
    }

    pub fn is_zero(&self) -> bool {
        self.state == 0
    }

    pub fn rotated_by(&self, rotation: &Rotation) -> Self {
        let state = match rotation {
            Rotation::Left(amount) => (self.state - amount).rem_euclid(100),
            Rotation::Right(amount) => (self.state + amount).rem_euclid(100),
        };

        Self { state }
    }
}
