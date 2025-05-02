use super::course::Course;
use super::history::Action;
use super::history::History;
use rand::{self, Rng};
use std::fmt::{self, Display, Formatter};
use std::fs::{self, File, create_dir_all};
use std::io::{self, Write};
use std::path::PathBuf;

pub struct CourseList {
    pub list: Vec<Course>,
    file: PathBuf,
    history: History,
}

impl CourseList {
    pub fn new<P>(path: P) -> CourseList
    where
        P: Into<PathBuf>,
    {
        CourseList {
            list: Vec::new(),
            file: path.into(),
            history: History::new(),
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
        match self._push(course.clone()) {
            Ok(_) => self.history.push(Action::Add(course)),
            Err(_) => eprintln!(
                "ERROR: While adding course: Duplicate course entry {}",
                course
            ),
        }
    }

    fn _push(&mut self, course: Course) -> Result<(), ()> {
        match self.list.binary_search(&course) {
            Ok(_) => Err(()),
            Err(index) => {
                self.list.insert(index, course);
                Ok(())
            }
        }
    }

    pub fn remove(&mut self, course: Course) {
        match self._remove(course.clone()) {
            Ok(_) => self.history.push(Action::Remove(course)),
            Err(_) => eprintln!(
                "ERROR While removing course: Course {} not found in list",
                course
            ),
        }
    }

    fn _remove(&mut self, course: Course) -> Result<(), ()> {
        match self.list.binary_search(&course) {
            Ok(index) => {
                self.list.remove(index);
                Ok(())
            }
            Err(_) => Err(()),
        }
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
        self.history.push(Action::Remove(self.list[to_pop].clone()));
        self.list.remove(to_pop);
        Some(())
    }

    pub fn get_history(&self) -> String {
        format!("{}", self.history)
    }

    pub fn roll_back(&mut self) {
        let res = self.history.back();
        self.undo_action(match res {
            Some(a) => a,
            None => {
                eprintln!("ERROR While rolling history back: ");
                return;
            }
        });
    }

    pub fn roll_forward(&mut self) {
        let res = self.history.forward();
        self.apply_action(match res {
            Some(a) => a,
            None => {
                eprintln!("ERROR While rolling history forward: ");
                return;
            }
        });
    }

    fn apply_action(&mut self, action: Action) {
        let _ = match action {
            Action::Add(c) => self._remove(c),
            Action::Remove(c) => self._push(c),
        };
    }

    fn undo_action(&mut self, action: Action) {
        let _ = match action {
            Action::Add(c) => self._remove(c),
            Action::Remove(c) => self._push(c),
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
    use tempfile::tempdir;

    #[test]
    fn test_new() {
        let file_path = tempdir().unwrap().path().join("test.json");
        let course_list = CourseList::new(&file_path);
        assert!(course_list.list.is_empty());
        assert_eq!(course_list.file, file_path);
        assert!(!course_list.history.has_history());
    }

    #[test]
    fn test_insert_in_order() {
        let file_path = tempdir().unwrap().path().join("test.json");
        let mut course_list = CourseList::new(&file_path);
        let course1 = Course::new(1, 111, "One");
        let course2 = Course::new(2, 112, "Two");
        let course3 = Course::new(3, 113, "Three");

        course_list._push(course1.clone()).unwrap();
        assert_eq!(course_list.list.len(), 1);
        assert_eq!(course_list.list[0], course1);

        course_list._push(course3.clone()).unwrap();
        assert_eq!(course_list.list.len(), 2);
        assert_eq!(course_list.list[0], course1);
        assert_eq!(course_list.list[1], course3);

        course_list._push(course2.clone()).unwrap();
        assert_eq!(course_list.list.len(), 3);
        assert_eq!(course_list.list[0], course1);
        assert_eq!(course_list.list[1], course2);
        assert_eq!(course_list.list[2], course3);

        assert!(course_list._push(course2.clone()).is_err());
        assert_eq!(course_list.list.len(), 3);
        assert_eq!(course_list.list[0], course1);
        assert_eq!(course_list.list[1], course2);
        assert_eq!(course_list.list[2], course3);
    }

    #[test]
    fn test_push() {
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
    fn test_remove() {
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
    fn test_dump_and_restore() {
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
    fn test_search_list() {
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
    fn test_generate() {
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
    fn test_display_list() {
        let file_path = tempdir().unwrap().path().join("test.json");
        let mut course_list = CourseList::new(&file_path);
        course_list.push(Course::new(1, 101, "One"));
        course_list.push(Course::new(2, 102, "Two"));
        assert_eq!(format!("{}", course_list), "(101, 01) One\n(102, 02) Two");
    }

    #[test]
    fn test_roll_forward_and_back() {
        let file_path = tempdir().unwrap().path().join("test.json");
        let mut course_list = CourseList::new(&file_path);

        let course1 = Course::new(1, 101, "One");
        let course2 = Course::new(2, 102, "Two");

        course_list.push(course1.clone());
        course_list.push(course2.clone());

        course_list.roll_back();
        assert_eq!(course_list.list.len(), 1);
        assert_eq!(course_list.list[0], course1);

        course_list.roll_back();
        assert_eq!(course_list.list.len(), 0);
    }
}
