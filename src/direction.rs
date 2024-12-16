#[derive(Debug, Default, Clone, Copy, PartialEq, PartialOrd, Eq, Ord)]
pub enum Direction {
    #[default]
    North,
    South,
    East,
    West,
}

impl Direction {
    pub fn turn_right(self) -> Self {
        match self {
            Self::North => Self::East,
            Self::East => Self::South,
            Self::South => Self::West,
            Self::West => Self::North,
        }
    }

    pub fn step(&self, coord: (usize, usize)) -> (usize, usize) {
        match self {
            Direction::North => (coord.0 - 1, coord.1),
            Direction::South => (coord.0 + 1, coord.1),
            Direction::East => (coord.0, coord.1 - 1),
            Direction::West => (coord.0, coord.1 + 1),
        }
    }
}
