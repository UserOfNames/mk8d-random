use rand::{self, Rng};
use serde::{Deserialize, Serialize};
use std::fmt::{self, Display, Formatter};
use std::fs::{self, File, create_dir_all};
use std::io::{self, Write};
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Course {
    name: String,
    coord: u32,
    rank: u8,
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

// An action the user takes, e.g. adding or removing a course
enum Action {
    Add(Course),
    Remove(Course),
}

pub struct CourseList {
    pub list: Vec<Course>,
    file: PathBuf,
    history: Vec<Action>,
}

impl CourseList {
    pub fn new<P>(path: P) -> CourseList
    where
        P: Into<PathBuf>,
    {
        CourseList {
            list: Vec::new(),
            file: path.into(),
            history: Vec::new(),
        }
    }

    pub fn dump_list(&self) -> io::Result<()> {
        let par = self.file.parent().unwrap();
        if !par.exists() {
            create_dir_all(par)?;
        }

        let mut file = File::create(&self.file)?;
        let data = serde_json::to_string_pretty(&self.list)?;
        file.write_all(data.as_bytes())?;
        Ok(())
    }

    pub fn restore_list(&mut self) -> io::Result<()> {
        let data = fs::read_to_string(&self.file)?;
        self.list = serde_json::from_str(&data)?;
        Ok(())
    }

    pub fn push(&mut self, course: Course) {
        self.insert_in_order(course);
    }

    pub fn remove(&mut self, course: Course) {
        match self.list.binary_search(&course) {
            Ok(index) => {
                self.list.remove(index);
            }

            Err(_) => eprintln!(
                "ERROR While removing course: Course {} not found in list",
                course
            ),
        };
    }

    fn search_list(&self, searched: &str) -> Vec<Course> {
        let mut res: Vec<Course> = Vec::new();

        for c in &self.list {
            if c.name.to_lowercase().contains(&searched.to_lowercase()) {
                res.push(c.clone());
            }
        }

        res
    }

    pub fn generate(&mut self) -> Option<()> {
        if self.list.len() == 0 {
            return None;
        }

        let to_pop: usize = rand::rng().random_range(0..self.list.len());
        println!("{}", self.list[to_pop]);
        self.list.remove(to_pop);
        Some(())
    }

    fn insert_in_order(&mut self, course: Course) {
        match self.list.binary_search(&course) {
            Ok(_) => eprintln!(
                "ERROR: While adding course: Duplicate course entry {}",
                course
            ),

            Err(index) => self.list.insert(index, course),
        };
    }
}

impl Display for CourseList {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let strings: Vec<String> = self.list.iter().map(|c| c.to_string()).collect();
        write!(f, "{}", strings.join("\n"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::tempdir;

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

    #[test]
    fn test_courselist_new() {
        let file_path = tempdir().unwrap().path().join("test.json");
        let course_list = CourseList::new(&file_path);
        assert!(course_list.list.is_empty());
        assert_eq!(course_list.file, file_path);
        assert!(course_list.history.is_empty());
    }

    #[test]
    fn test_courselist_insert_in_order() {
        let file_path = tempdir().unwrap().path().join("test.json");
        let mut course_list = CourseList::new(&file_path);
        let course1 = Course::new(1, 111, "One");
        let course2 = Course::new(2, 112, "Two");
        let course3 = Course::new(3, 113, "Three");

        course_list.insert_in_order(course1.clone());
        assert_eq!(course_list.list.len(), 1);
        assert_eq!(course_list.list[0], course1);

        course_list.insert_in_order(course3.clone());
        assert_eq!(course_list.list.len(), 2);
        assert_eq!(course_list.list[0], course1);
        assert_eq!(course_list.list[1], course3);

        course_list.insert_in_order(course2.clone());
        assert_eq!(course_list.list.len(), 3);
        assert_eq!(course_list.list[0], course1);
        assert_eq!(course_list.list[1], course2);
        assert_eq!(course_list.list[2], course3);

        course_list.insert_in_order(course2.clone());
        assert_eq!(course_list.list.len(), 3);
        assert_eq!(course_list.list[0], course1);
        assert_eq!(course_list.list[1], course2);
        assert_eq!(course_list.list[2], course3);
    }

    #[test]
    fn test_courselist_push() {
        let file_path = tempdir().unwrap().path().join("test.json");
        let mut course_list = CourseList::new(&file_path);
        let course1 = Course::new(1, 111, "One");
        let course2 = Course::new(2, 112, "Two");
        course_list.push(course1.clone());
        course_list.push(course2.clone());
        assert_eq!(course_list.list.len(), 2);
        assert_eq!(course_list.list[0], course1);
        assert_eq!(course_list.list[1], course2);
    }

    #[test]
    fn test_courselist_remove() {
        let file_path = tempdir().unwrap().path().join("test.json");
        let mut course_list = CourseList::new(&file_path);
        let course1 = Course::new(1, 111, "One");
        let course2 = Course::new(2, 112, "Two");
        course_list.push(course1.clone());
        course_list.push(course2.clone());
        course_list.remove(course1.clone());
        assert_eq!(course_list.list.len(), 1);
        assert_eq!(course_list.list[0], course2);
        course_list.remove(course1.clone());
        assert_eq!(course_list.list.len(), 1);
        assert_eq!(course_list.list[0], course2);
        course_list.remove(course2.clone());
        assert_eq!(course_list.list.len(), 0);
        course_list.remove(course2.clone());
        assert_eq!(course_list.list.len(), 0);
    }

    #[test]
    fn test_courselist_dump_and_restore() {
        let file_path = tempdir().unwrap().path().join("test.json");
        let mut course_list = CourseList::new(&file_path);
        course_list.push(Course::new(1, 111, "One"));
        course_list.push(Course::new(2, 112, "Two"));

        course_list.dump_list().expect("Failed to dump list");

        let mut restored_list = CourseList::new(&file_path);
        restored_list
            .restore_list()
            .expect("Failed to restore list");

        assert_eq!(course_list.list.len(), restored_list.list.len());
        assert_eq!(course_list.list[0].rank, restored_list.list[0].rank);
        assert_eq!(course_list.list[1].name, restored_list.list[1].name);
    }

    #[test]
    fn test_courselist_search_list() {
        let file_path = tempdir().unwrap().path().join("test.json");
        let mut course_list = CourseList::new(&file_path);
        course_list.push(Course::new(1, 101, "One"));
        course_list.push(Course::new(2, 102, "Two"));
        course_list.push(Course::new(3, 103, "Three"));

        let results = course_list.search_list("t");
        assert_eq!(results.len(), 2);
        assert_eq!(results[0].name, "Two");
        assert_eq!(results[1].name, "Three");

        let empty_results = course_list.search_list("A");
        assert!(empty_results.is_empty());
    }

    #[test]
    fn test_courselist_generate() {
        let file_path = tempdir().unwrap().path().join("test.json");
        let mut course_list = CourseList::new(&file_path);
        course_list.push(Course::new(1, 101, "One"));
        course_list.push(Course::new(2, 102, "Two"));

        assert!(course_list.generate().is_some());
        assert_eq!(course_list.list.len(), 1);

        assert!(course_list.generate().is_some());
        assert_eq!(course_list.list.len(), 0);
    }

    #[test]
    fn test_courselist_display_list() {
        let file_path = tempdir().unwrap().path().join("test.json");
        let mut course_list = CourseList::new(&file_path);
        course_list.push(Course::new(1, 101, "One"));
        course_list.push(Course::new(2, 102, "Two"));
        assert_eq!(format!("{}", course_list), "(101, 01) One\n(102, 02) Two");
    }
}
