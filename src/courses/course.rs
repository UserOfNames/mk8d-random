use std::fmt::{self, Display, Formatter};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Coord {
    row: u8,
    col: u8,
    pos: u8,
}

impl Coord {
    #[allow(unused)]
    pub fn new(row: u8, col: u8, pos: u8) -> Self {
        Coord { row, col, pos }
    }
}

impl Display for Coord {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}-{}-{}", self.row, self.col, self.pos)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Course {
    pub name: String,
    pub coord: Coord,
    pub rank: u8,
}

impl Course {
    #[allow(unused)]
    pub fn new(rank: u8, coord: Coord, name: &str) -> Self {
        Course {
            name: name.to_string(),
            coord,
            rank,
        }
    }
}

impl Display for Course {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {:02}) {}", self.coord, self.rank, self.name)
    }
}

impl PartialEq for Course {
    fn eq(&self, other: &Self) -> bool {
        self.rank == other.rank
    }
}

impl Eq for Course {}

impl PartialOrd for Course {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Course {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.rank.cmp(&other.rank)
    }
}
