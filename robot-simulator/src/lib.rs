// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

#[derive(PartialEq, Eq, Debug)]
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
        Robot { x: x, y: y, d: d }
    }

    #[must_use]
    pub fn turn_right(self) -> Self {
        let mut ret = self;
        ret.d = match ret.d {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        };
        ret
    }

    #[must_use]
    pub fn turn_left(self) -> Self {
        let mut ret = self;
        ret.d = match ret.d {
            Direction::North => Direction::West,
            Direction::East => Direction::North,
            Direction::South => Direction::East,
            Direction::West => Direction::South,
        };
        ret
    }

    #[must_use]
    pub fn advance(self) -> Self {
        let mut ret = self;
        (ret.x, ret.y) = match ret.d {
            Direction::North => (ret.x, ret.y + 1),
            Direction::East => (ret.x + 1, ret.y),
            Direction::South => (ret.x, ret.y - 1),
            Direction::West => (ret.x - 1, ret.y),
        };
        ret
    }

    #[must_use]
    pub fn instructions(self, instructions: &str) -> Self {
        let ret = self;
        instructions.chars().fold(ret, |ret, op| match op {
            'R' => ret.turn_right(),
            'L' => ret.turn_left(),
            'A' => ret.advance(),
            _ => ret,
        })
    }

    pub fn position(&self) -> (i32, i32) {
        (self.x, self.y)
    }

    pub fn direction(&self) -> &Direction {
        &self.d
    }
}
