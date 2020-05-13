#[derive(PartialEq, Debug)]
pub enum Direction {
    North = 0,
    East,
    South,
    West,
}

use Direction::{East, North, South, West};

impl From<i32> for Direction {
    fn from(d: i32) -> Direction {
        match (d + 4) % 4 {
            0 => North,
            1 => East,
            2 => South,
            3 => West,
            _ => panic!("What?.."),
        }
    }
}

pub struct Robot {
    x: i32,
    y: i32,
    d: Direction,
}

impl Robot {
    pub fn new(x: i32, y: i32, d: Direction) -> Self {
        Self { x, y, d }
    }

    pub fn turn_right(mut self) -> Self {
        self.d = Direction::from(self.d as i32 + 1);
        self
    }

    pub fn turn_left(mut self) -> Self {
        self.d = Direction::from(self.d as i32 - 1);
        self
    }

    pub fn advance(mut self) -> Self {
        match self.d {
            North => self.y += 1,
            East => self.x += 1,
            South => self.y -= 1,
            West => self.x -= 1,
        }
        self
    }

    pub fn instructions(mut self, instructions: &str) -> Self {
        for c in instructions.chars() {
            self = match c {
                'A' => self.advance(),
                'R' => self.turn_right(),
                'L' => self.turn_left(),
                _ => panic!("Wrong instruction set."),
            };
        }
        self
    }

    pub fn position(&self) -> (i32, i32) {
        (self.x, self.y)
    }

    pub fn direction(&self) -> &Direction {
        &self.d
    }
}
