#[derive(PartialEq, Debug)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

pub struct Robot {
    coordinates: (i32, i32),
    direction: Direction,
}

impl Robot {
    pub fn new(x: i32, y: i32, d: Direction) -> Self {
        Robot {
            coordinates: (x, y),
            direction: d,
        }
    }

    pub fn turn_right(self) -> Self {
        let direction = match self.direction {
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
            Direction::North => Direction::East,
        };
        Robot { direction, ..self }
    }

    pub fn turn_left(self) -> Self {
        let direction = match self.direction {
            Direction::East => Direction::North,
            Direction::South => Direction::East,
            Direction::West => Direction::South,
            Direction::North => Direction::West,
        };
        Robot { direction, ..self }
    }

    pub fn advance(self) -> Self {
        let coordinates = match self.direction {
            Direction::East => (self.coordinates.0 + 1, self.coordinates.1),
            Direction::South => (self.coordinates.0, self.coordinates.1 - 1),
            Direction::West => (self.coordinates.0 - 1, self.coordinates.1),
            Direction::North => (self.coordinates.0, self.coordinates.1 + 1),
        };
        Robot {
            coordinates,
            ..self
        }
    }

    pub fn instructions(self, instructions: &str) -> Self {
        instructions.chars().fold(self, |acc, ch| match ch {
            'L' => acc.turn_left(),
            'R' => acc.turn_right(),
            'A' => acc.advance(),
            _ => unreachable!("Not a valid instruction."),
        })
    }

    pub fn position(&self) -> (i32, i32) {
        self.coordinates
    }

    pub fn direction(&self) -> &Direction {
        &self.direction
    }
}
