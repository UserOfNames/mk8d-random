use std::fmt::{self, Display, Formatter};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Course {
    pub name: String,
    pub coord: u32,
    pub rank: u8,
}

impl Course {
    pub fn new(rank: u8, coord: u32, name: &str) -> Self {
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
        self.rank.partial_cmp(&other.rank)
    }
}

impl Ord for Course {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.rank.cmp(&other.rank)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_course_new() {
        let course = Course::new(1, 111, "Test1");
        assert_eq!(course.rank, 1);
        assert_eq!(course.coord, 111);
        assert_eq!(course.name, "Test1");
    }

    #[test]
    fn test_course_display() {
        let course = Course::new(2, 112, "Test2");
        assert_eq!(format!("{}", course), "(112, 02) Test2");
    }

    #[test]
    fn test_course_partial_eq() {
        let course = Course::new(3, 113, "Test3a");
        let course_eq = Course::new(3, 999, "whatever");
        let course_not_eq = Course::new(4, 113, "Test3b");
        assert_eq!(course, course_eq);
        assert_ne!(course, course_not_eq);
    }

    #[test]
    fn test_course_ordering() {
        let course1 = Course::new(3, 113, "Test4a");
        let course2 = Course::new(4, 999, "Test4b");
        assert!(course1 < course2);
        assert!(course2 > course1);
        assert!(!(course1 > course2));
        assert!(!(course2 < course1));
    }
}
