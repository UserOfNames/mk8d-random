//! Module defining the `Course` struct and its components.

use std::fmt::{self, Display, Formatter};

use serde::{Deserialize, Serialize};

/// Game from which a given `Course` originated.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[allow(missing_docs)]
#[non_exhaustive]
pub enum Origin {
    SNES,
    GBA,
    N64,
    GCN,
    DS,
    Wii,
    TDS,
    Tour,
    MK8,
}

impl Display for Origin {
    #[rustfmt::skip]
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::SNES => "SNES",
            Self::GBA => "GBA",
            Self::N64 => "N64",
            Self::GCN => "GCN",
            Self::DS => "DS",
            Self::Wii => "Wii",
            Self::TDS => "3DS",
            Self::Tour => "Tour",
            Self::MK8 => "MK8",
        }.fmt(f)
    }
}

/// Defines a coordinate in the selection screen: row and column give a cup, and position is the
/// number of the course in that cup.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Coord {
    row: u8,
    col: u8,
    pos: u8,
}

impl Coord {
    /// Create a new coordinate.
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

/// A course.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Course {
    /// The course's name, e.g. "Rainbow Road."
    pub name: String,
    /// The game from which the course originated.
    pub origin: Origin,
    /// The course's position in the selection screen.
    pub coord: Coord,
    /// The course's rank quality ranking relative to other courses. This should be unique,
    /// descending, and start at 1. If there are N courses in the game, 1 is the best and N is the
    /// worst.
    pub rank: usize,
}

impl Course {
    /// Create a new course.
    #[allow(unused)]
    pub fn new(rank: usize, coord: Coord, origin: Origin, name: &str) -> Self {
        Course {
            name: name.to_owned(),
            origin,
            coord,
            rank,
        }
    }
}

impl Display for Course {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "({coord}, {rank:02}) {origin:<4} {name}",
            coord = self.coord,
            rank = self.rank,
            origin = self.origin,
            name = self.name,
        )
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
