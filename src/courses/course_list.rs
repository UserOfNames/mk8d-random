use std::collections::BTreeSet;
use std::fs::{self, File, create_dir_all};
use std::io::{self, Write};
use std::path::PathBuf;

use rand::seq::IndexedRandom;
use rand::{self, Rng};
use serde::{Deserialize, Serialize};

use crate::SAVES_DIR;

use super::course::Course;
use super::history::Action;
use super::history::History;

#[derive(Serialize, Deserialize, Debug)]
pub struct CourseList {
    current: BTreeSet<Course>,
    removed: BTreeSet<Course>,
    save_name: PathBuf,
    history: History,
}

impl CourseList {
    pub fn new(save_name: impl Into<PathBuf>) -> Self {
        CourseList {
            current: BTreeSet::new(),
            removed: BTreeSet::new(),
            save_name: save_name.into(),
            history: History::new(),
        }
    }

    pub fn restore_save(save_name: impl Into<PathBuf>) -> io::Result<Self> {
        let data = fs::read_to_string(SAVES_DIR.join(save_name.into()))?;
        Ok(serde_json::from_str(&data)?)
    }

    pub fn dump_list(&self) -> io::Result<()> {
        let path = self.path();
        let par = path.parent().unwrap();
        if !par.exists() {
            create_dir_all(par)?;
        }

        let data = serde_json::to_string_pretty(&self)?;
        let mut file = File::create(path)?;
        file.write_all(data.as_bytes())?;
        Ok(())
    }

    // pub fn restore_self(&mut self) -> io::Result<()> {
    //     let data = fs::read_to_string(self.path())?;
    //     *self = serde_json::from_str(&data)?;
    //     Ok(())
    // }

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

    pub fn get_random_by_chunks(
        &self,
        num_chunks: usize,
    ) -> Result<impl Iterator<Item = Course>, ()> {
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

    pub fn path(&self) -> PathBuf {
        SAVES_DIR.join(&self.save_name)
    }
}
