// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

#[derive(PartialEq, Debug)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

pub struct Robot {
    x: i32,
    y: i32,
    d: Direction,
}

impl Robot {
    pub fn new(x: i32, y: i32, d: Direction) -> Self {
        Robot{x, y, d}
    }

    pub fn turn_right(self) -> Self {
        let d = match self.d {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        };
        Robot{x: self.x, y: self.y, d}
    }

    pub fn turn_left(self) -> Self {
        let d = match self.d {
            Direction::North => Direction::West,
            Direction::West => Direction::South,
            Direction::South => Direction::East,
            Direction::East => Direction::North,
        };
        Robot{x: self.x, y: self.y, d}
    }

    pub fn advance(self) -> Self {
        let (dx, dy) = match self.d {
            Direction::North => (0, 1),
            Direction::East => (1, 0),
            Direction::South => (0, -1),
            Direction::West => (-1, 0),
        };
        Robot{x: self.x + dx, y: self.y + dy, d: self.d }
    }

    pub fn instructions(self, instructions: &str) -> Self {
        let mut current = self;
        for instruction in instructions.chars() {
            current = match instruction {
                'A' => current.advance(),
                'L' => current.turn_left(),
                'R' => current.turn_right(),
                _ => panic!("no such char")
            }
        }
        current
    }

    pub fn position(&self) -> (i32, i32) {
        (self.x, self.y)
    }

    pub fn direction(&self) -> &Direction {
        &self.d
    }
}
