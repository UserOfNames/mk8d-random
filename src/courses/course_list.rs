use super::course::Course;
use super::history::Action;
use super::history::History;
use rand::{self, Rng};
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;
use std::fmt::{self, Display, Formatter};
use std::fs::{self, File, create_dir_all};
use std::io::{self, Write};
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Debug)]
pub struct CourseList {
    pub current: BTreeSet<Course>,
    pub removed: BTreeSet<Course>,
    file: PathBuf,
    history: History,
}

impl CourseList {
    pub fn new(path: impl Into<PathBuf>) -> Self {
        CourseList {
            current: BTreeSet::new(),
            removed: BTreeSet::new(),
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
        let data = serde_json::to_string_pretty(&self)?;
        file.write_all(data.as_bytes())?;
        Ok(())
    }

    pub fn restore_list(&mut self) -> io::Result<()> {
        let data = fs::read_to_string(&self.file)?;
        *self = serde_json::from_str(&data)?;
        Ok(())
    }

    pub fn add(&mut self, course: Course) -> Result<(), ()> {
        self._add(course.clone())?;
        self.history.push(Action::Add(course));
        Ok(())
    }

    fn _add(&mut self, course: Course) -> Result<(), ()> {
        self.removed.remove(&course);
        if self.current.insert(course.clone()) {
            Ok(())
        } else {
            Err(())
        }
    }

    pub fn remove(&mut self, course: Course) -> Result<(), ()> {
        self._remove(course.clone())?;
        self.history.push(Action::Remove(course));
        Ok(())
    }

    fn _remove(&mut self, course: Course) -> Result<(), ()> {
        self.current.remove(&course);
        if self.removed.insert(course.clone()) {
            Ok(())
        } else {
            Err(())
        }
    }

    fn search_current(&self, searched: &str) -> BTreeSet<Course> {
        self.current
            .iter()
            .filter(|c| c.name.to_lowercase().contains(&searched.to_lowercase()))
            .cloned()
            .collect()
    }

    fn search_removed(&self, searched: &str) -> BTreeSet<Course> {
        self.removed
            .iter()
            .filter(|c| c.name.to_lowercase().contains(&searched.to_lowercase()))
            .cloned()
            .collect()
    }

    pub fn generate(&mut self) -> Option<Course> {
        let index: usize = rand::rng().random_range(0..self.current.len());
        let to_remove = self.current.iter().nth(index).cloned();
        if let Some(c) = &to_remove {
            self.remove(c.clone());
        }
        to_remove
    }

    pub fn reset(&mut self) {
        self.current.append(&mut self.removed);
    }

    pub fn get_history(&self) -> &History {
        &self.history
    }

    pub fn roll_back(&mut self) -> Result<(), ()> {
        match self.history.back() {
            Some(action) => {
                self.undo_action(action);
                Ok(())
            }

            None => Err(()),
        }
    }

    pub fn roll_forward(&mut self) -> Result<(), ()> {
        match self.history.forward() {
            Some(action) => {
                self.apply_action(action);
                Ok(())
            }

            None => Err(()),
        }
    }

    fn apply_action(&mut self, action: Action) {
        let _ = match action {
            Action::Add(c) => self._add(c),
            Action::Remove(c) => self._remove(c),
        };
    }

    fn undo_action(&mut self, action: Action) {
        let _ = match action {
            Action::Add(c) => self._remove(c),
            Action::Remove(c) => self._add(c),
        };
    }
}

impl Display for CourseList {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let strings: Vec<String> = self.current.iter().map(|c| c.to_string()).collect();
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
        assert!(course_list.current.is_empty());
        assert_eq!(course_list.file, file_path);
        assert!(!course_list.history.has_history());
    }

    #[test]
    fn test_add() {
        let file_path = tempdir().unwrap().path().join("test.json");
        let mut course_list = CourseList::new(&file_path);
        let course1 = Course::new(1, 111, "One");
        let course2 = Course::new(2, 112, "Two");

        // Because add() expects to remove from self.removed, we have to prepare it
        course_list.removed.insert(course1.clone());
        course_list.removed.insert(course2.clone());

        assert!(course_list.add(course1.clone()).is_ok());
        assert!(course_list.add(course2.clone()).is_ok());
        let mut current = course_list.current.iter();

        assert_eq!(course_list.current.len(), 2);
        assert_eq!(*current.next().unwrap(), course1);
        assert_eq!(*current.next().unwrap(), course2);
    }

    #[test]
    fn test_remove() {
        let file_path = tempdir().unwrap().path().join("test.json");
        let mut course_list = CourseList::new(&file_path);
        let course1 = Course::new(1, 111, "One");
        let course2 = Course::new(2, 112, "Two");

        // Because add() expects to remove from self.removed, we have to prepare it
        course_list.removed.insert(course1.clone());
        course_list.removed.insert(course2.clone());

        assert!(course_list.add(course1.clone()).is_ok());
        assert!(course_list.add(course2.clone()).is_ok());

        assert!(course_list.remove(course1.clone()).is_ok());
        assert_eq!(course_list.current.len(), 1);
        assert_eq!(course_list.removed.len(), 1);
        assert_eq!(*course_list.current.iter().next().unwrap(), course2);
        assert_eq!(*course_list.removed.iter().next().unwrap(), course1);

        assert!(course_list.remove(course1.clone()).is_err());
        assert_eq!(course_list.current.len(), 1);
        assert_eq!(course_list.removed.len(), 1);
        assert_eq!(*course_list.current.iter().next().unwrap(), course2);
        assert_eq!(*course_list.removed.iter().next().unwrap(), course1);

        assert!(course_list.remove(course2.clone()).is_ok());
        assert_eq!(course_list.current.len(), 0);
        assert_eq!(course_list.removed.len(), 2);
        assert_eq!(*course_list.removed.iter().next().unwrap(), course1);

        assert!(!course_list.remove(course2.clone()).is_ok());
        assert_eq!(course_list.current.len(), 0);
        assert_eq!(course_list.removed.len(), 2);
        assert_eq!(*course_list.removed.iter().next().unwrap(), course1);
    }

    #[test]
    fn test_dump_and_restore() {
        let file_path = tempdir().unwrap().path().join("test.json");
        let mut course_list = CourseList::new(&file_path);

        let course1 = Course::new(1, 111, "One");
        let course2 = Course::new(2, 112, "Two");

        course_list.add(course1).unwrap();
        course_list.add(course2).unwrap();

        course_list.dump_list().expect("Failed to dump list");

        let mut restored_list = CourseList::new(&file_path);
        restored_list
            .restore_list()
            .expect("Failed to restore list");

        assert_eq!(course_list.current.len(), restored_list.current.len());
        let mut current = course_list.current.iter();
        let mut restored = course_list.current.iter();
        assert_eq!(current.next().unwrap().rank, restored.next().unwrap().rank);
        assert_eq!(current.next().unwrap().name, restored.next().unwrap().name);
    }

    #[test]
    fn test_search_current_and_removed() {
        let file_path = tempdir().unwrap().path().join("test.json");
        let mut course_list = CourseList::new(&file_path);

        let course1 = Course::new(1, 101, "One");
        let course2 = Course::new(2, 102, "Two");
        let course3 = Course::new(3, 103, "Three");
        let course4 = Course::new(4, 104, "Four");

        course_list.add(course1).unwrap();
        course_list.add(course2).unwrap();
        course_list.add(course3).unwrap();
        course_list.add(course4.clone()).unwrap();
        course_list.remove(course4).unwrap();

        let mut current = course_list.search_current("t").into_iter();
        let mut removed = course_list.search_removed("f").into_iter();

        assert_eq!(current.len(), 2);
        assert_eq!(current.next().unwrap().name, "Two");
        assert_eq!(current.next().unwrap().name, "Three");
        assert_eq!(removed.next().unwrap().name, "Four");

        let empty_results = course_list.search_current("A");
        assert!(empty_results.is_empty());
    }

    #[test]
    fn test_generate() {
        let file_path = tempdir().unwrap().path().join("test.json");
        let mut course_list = CourseList::new(&file_path);

        let course1 = Course::new(1, 101, "One");
        let course2 = Course::new(2, 102, "Two");

        course_list.add(course1.clone()).unwrap();
        course_list.add(course2.clone()).unwrap();

        assert!(course_list.generate().is_some());
        assert_eq!(course_list.current.len(), 1);

        assert!(course_list.generate().is_some());
        assert_eq!(course_list.current.len(), 0);
    }

    #[test]
    fn test_display_list() {
        let file_path = tempdir().unwrap().path().join("test.json");
        let mut course_list = CourseList::new(&file_path);

        course_list.add(Course::new(1, 101, "One")).unwrap();
        course_list.add(Course::new(2, 102, "Two")).unwrap();
        assert_eq!(course_list.to_string(), "(101, 01) One\n(102, 02) Two");
    }

    #[test]
    fn test_roll_forward_and_back() {
        let file_path = tempdir().unwrap().path().join("test.json");
        let mut course_list = CourseList::new(&file_path);

        let course1 = Course::new(1, 101, "One");
        let course2 = Course::new(2, 102, "Two");

        course_list.add(course1.clone()).unwrap();
        course_list.add(course2.clone()).unwrap();

        course_list.roll_back().unwrap();
        assert_eq!(course_list.current.len(), 1);
        assert_eq!(*course_list.current.iter().next().unwrap(), course1);

        course_list.roll_back().unwrap();
        println!("{:?}", course_list);
        assert_eq!(course_list.current.len(), 0);

        assert!(course_list.roll_back().is_err());

        course_list.roll_forward().unwrap();
        assert_eq!(course_list.current.len(), 1);
    }

    #[test]
    fn test_get_history() {
        let file_path = tempdir().unwrap().path().join("test.json");
        let mut course_list = CourseList::new(&file_path);

        let mut hist = course_list.get_history();
        assert!(!hist.has_history());

        let course1 = Course::new(1, 101, "One");
        let course2 = Course::new(2, 102, "Two");
        let course3 = Course::new(3, 103, "Three");

        course_list.add(course1.clone()).unwrap();
        course_list.add(course2.clone()).unwrap();
        course_list.add(course3.clone()).unwrap();
        course_list.remove(course2.clone()).unwrap();

        hist = course_list.get_history();
        assert!(hist.has_history());
    }

    #[test]
    fn test_reset() {
        let file_path = tempdir().unwrap().path().join("test.json");
        let mut course_list = CourseList::new(&file_path);

        let course1 = Course::new(1, 101, "One");
        let course2 = Course::new(2, 102, "Two");
        let course3 = Course::new(3, 103, "Three");

        course_list.add(course1.clone()).unwrap();
        course_list.add(course2.clone()).unwrap();
        course_list.add(course3.clone()).unwrap();
        course_list.remove(course2.clone()).unwrap();
        assert_eq!(course_list.current.len(), 2);
        assert_eq!(course_list.removed.len(), 1);
        assert_eq!(*course_list.current.first().unwrap(), course1);
        assert_eq!(*course_list.current.last().unwrap(), course3);
        assert_eq!(*course_list.removed.first().unwrap(), course2);

        course_list.reset();
        assert_eq!(course_list.current.len(), 3);
        assert_eq!(course_list.removed.len(), 0);
        assert_eq!(*course_list.current.first().unwrap(), course1);
        assert_eq!(*course_list.current.last().unwrap(), course3);
    }
}
