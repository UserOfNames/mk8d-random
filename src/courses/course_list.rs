use std::collections::BTreeSet;
use std::fs::{self, File, create_dir_all};
use std::io::{self, Write};
use std::path::PathBuf;

use rand::seq::IndexedRandom;
use rand::{self, Rng};
use serde::{Deserialize, Serialize};

use super::course::Course;
use super::history::Action;
use super::history::History;

#[derive(Serialize, Deserialize, Debug)]
pub struct CourseList {
    current: BTreeSet<Course>,
    removed: BTreeSet<Course>,
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

    pub fn new_with_list(path: impl Into<PathBuf>, list: BTreeSet<Course>) -> Self {
        CourseList {
            current: list,
            removed: BTreeSet::new(),
            file: path.into(),
            history: History::new(),
        }
    }

    pub fn restore_save(path: impl Into<PathBuf>) -> io::Result<Self> {
        let data = fs::read_to_string(path.into())?;
        Ok(serde_json::from_str(&data)?)
    }

    pub fn dump_list(&self) -> io::Result<()> {
        let par = self.file.parent().unwrap();
        if !par.exists() {
            create_dir_all(par)?;
        }

        let data = serde_json::to_string_pretty(&self)?;
        let mut file = File::create(&self.file)?;
        file.write_all(data.as_bytes())?;
        Ok(())
    }

    pub fn restore_self(&mut self) -> io::Result<()> {
        let data = fs::read_to_string(&self.file)?;
        *self = serde_json::from_str(&data)?;
        Ok(())
    }

    pub fn add(&mut self, course: Course) {
        self._add(course.clone());
        self.history.push(Action::Add(course));
    }

    fn _add(&mut self, course: Course) {
        self.removed.remove(&course);
        self.current.insert(course);
    }

    pub fn remove(&mut self, course: Course) {
        self._remove(course.clone());
        self.history.push(Action::Remove(course));
    }

    fn _remove(&mut self, course: Course) {
        self.current.remove(&course);
        self.removed.insert(course);
    }

    pub fn search_current(&self, searched: &str) -> impl Iterator<Item = &Course> {
        let key = searched.to_lowercase();
        self.current
            .iter()
            .filter(move |c| c.name.to_lowercase().contains(&key))
    }

    pub fn search_removed(&self, searched: &str) -> impl Iterator<Item = &Course> {
        let key = searched.to_lowercase();
        self.removed
            .iter()
            .filter(move |c| c.name.to_lowercase().contains(&key))
    }

    pub fn get_random(&self) -> Option<&Course> {
        if self.current.is_empty() {
            return None;
        };

        let index: usize = rand::rng().random_range(0..self.current.len());
        self.current.iter().nth(index)
    }

    pub fn get_random_by_chunks(&self, num_chunks: usize) -> Result<impl Iterator<Item = Course>, ()> {
        let curr_vec: Vec<&Course> = self.current.iter().collect();
        let len = self.current.len();

        if len % num_chunks != 0 {
            return Err(());
        }
        let chunk_size = len / num_chunks;

        let mut rng = rand::rng();
        let mut res: Vec<Course> = Vec::with_capacity(num_chunks);

        for chunk in curr_vec.chunks_exact(chunk_size) {
            // We already validated the chunks, so unwrap() is fine here
            let selection = *chunk.choose(&mut rng).unwrap();
            res.push(selection.clone());
        }

        Ok(res.into_iter())
    }

    pub fn reset(&mut self) {
        self.current.append(&mut self.removed);
        self.history.reset();
    }

    pub fn get_history(&self) -> &History {
        &self.history
    }

    pub fn get_current(&self) -> &BTreeSet<Course> {
        &self.current
    }

    pub fn get_removed(&self) -> &BTreeSet<Course> {
        &self.removed
    }

    pub fn roll_back(&mut self) -> Result<(), ()> {
        let action: Action = self.history.back().ok_or(())?;
        self.undo_action(action);
        Ok(())
    }

    pub fn roll_forward(&mut self) -> Result<(), ()> {
        let action: Action = self.history.forward().ok_or(())?;
        self.apply_action(action);
        Ok(())
    }

    fn apply_action(&mut self, action: Action) {
        match action {
            Action::Add(c) => self._add(c),
            Action::Remove(c) => self._remove(c),
        };
    }

    fn undo_action(&mut self, action: Action) {
        match action {
            Action::Add(c) => self._remove(c),
            Action::Remove(c) => self._add(c),
        };
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

        course_list.add(course1.clone());
        course_list.add(course2.clone());
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

        course_list.add(course1.clone());
        course_list.add(course2.clone());

        course_list.remove(course1.clone());
        assert_eq!(course_list.current.len(), 1);
        assert_eq!(course_list.removed.len(), 1);
        assert_eq!(*course_list.current.iter().next().unwrap(), course2);
        assert_eq!(*course_list.removed.iter().next().unwrap(), course1);

        course_list.remove(course1.clone());
        assert_eq!(course_list.current.len(), 1);
        assert_eq!(course_list.removed.len(), 1);
        assert_eq!(*course_list.current.iter().next().unwrap(), course2);
        assert_eq!(*course_list.removed.iter().next().unwrap(), course1);

        course_list.remove(course2.clone());
        assert_eq!(course_list.current.len(), 0);
        assert_eq!(course_list.removed.len(), 2);
        assert_eq!(*course_list.removed.iter().next().unwrap(), course1);

        course_list.remove(course2.clone());
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

        course_list.add(course1);
        course_list.add(course2);

        course_list.dump_list().expect("Failed to dump list");

        let mut restored_list = CourseList::new(&file_path);
        restored_list
            .restore_self()
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

        course_list.add(course1);
        course_list.add(course2);
        course_list.add(course3);
        course_list.add(course4.clone());
        course_list.remove(course4);

        let mut current = course_list.search_current("t");
        let mut removed = course_list.search_removed("f");

        assert_eq!(current.next().unwrap().name, "Two");
        assert_eq!(current.next().unwrap().name, "Three");
        assert!(current.next().is_none());
        assert_eq!(removed.next().unwrap().name, "Four");

        let mut empty_results = course_list.search_current("A");
        assert!(empty_results.next().is_none());
    }

    #[test]
    fn test_generate() {
        let file_path = tempdir().unwrap().path().join("test.json");
        let mut course_list = CourseList::new(&file_path);

        let course1 = Course::new(1, 101, "One");
        let course2 = Course::new(2, 102, "Two");

        course_list.add(course1.clone());
        course_list.add(course2.clone());

        assert_eq!(course_list.current.len(), 2);

        let a = course_list.get_random().unwrap();
        course_list.remove(a.clone());
        assert_eq!(course_list.current.len(), 1);

        let b = course_list.get_random().unwrap();
        course_list.remove(b.clone());
        assert_eq!(course_list.current.len(), 0);
    }

    #[test]
    fn test_roll_forward_and_back() {
        let file_path = tempdir().unwrap().path().join("test.json");
        let mut course_list = CourseList::new(&file_path);

        let course1 = Course::new(1, 101, "One");
        let course2 = Course::new(2, 102, "Two");

        course_list.add(course1.clone());
        course_list.add(course2.clone());

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
    fn test_reset() {
        let file_path = tempdir().unwrap().path().join("test.json");
        let mut course_list = CourseList::new(&file_path);

        let course1 = Course::new(1, 101, "One");
        let course2 = Course::new(2, 102, "Two");
        let course3 = Course::new(3, 103, "Three");

        course_list.add(course1.clone());
        course_list.add(course2.clone());
        course_list.add(course3.clone());
        course_list.remove(course2.clone());
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
