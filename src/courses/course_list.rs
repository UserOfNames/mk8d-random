use std::collections::BTreeSet;
use std::fs::{self, File, create_dir_all};
use std::io::{self, Write};
use std::path::PathBuf;

use rand::{self, Rng, seq::IndexedRandom};
use serde::{Deserialize, Serialize};

use crate::SAVES_DIR;

use super::course::Course;
use super::history::Action;
use super::history::History;

#[derive(Serialize, Deserialize, Debug)]
pub struct CourseList {
    pub save_name: PathBuf,
    pub courses: Vec<Course>,
    current: BTreeSet<usize>,
    removed: BTreeSet<usize>,
    history: History,
}

impl CourseList {
    pub fn new(save_name: impl Into<PathBuf>) -> Self {
        CourseList {
            courses: Vec::new(),
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

    pub fn add(&mut self, course_i: usize) {
        self._add(course_i);
        self.history.push(Action::Add(course_i));
    }

    fn _add(&mut self, course_i: usize) {
        self.removed.remove(&course_i);
        self.current.insert(course_i);
    }

    pub fn remove(&mut self, course_i: usize) {
        self._remove(course_i);
        self.history.push(Action::Remove(course_i));
    }

    fn _remove(&mut self, course_i: usize) {
        self.current.remove(&course_i);
        self.removed.insert(course_i);
    }

    pub fn search_current(&self, searched: &str) -> impl Iterator<Item = usize> {
        let key = searched.to_lowercase();
        self.current
            .iter()
            .copied()
            .filter(move |&i| self.courses[i].name.to_lowercase().contains(&key))
    }

    pub fn search_removed(&self, searched: &str) -> impl Iterator<Item = usize> {
        let key = searched.to_lowercase();
        self.removed
            .iter()
            .copied()
            .filter(move |&i| self.courses[i].name.to_lowercase().contains(&key))
    }

    pub fn get_random(&self) -> Option<usize> {
        if self.current.is_empty() {
            return None;
        };

        let index: usize = rand::rng().random_range(0..self.current.len());
        self.current.iter().nth(index).copied()
    }

    pub fn get_random_by_chunks(
        &self,
        num_chunks: usize,
    ) -> Result<impl Iterator<Item = usize>, ()> {
        let curr_vec: Vec<usize> = self.current.iter().copied().collect();
        let len = self.current.len();

        if len % num_chunks != 0 {
            return Err(());
        }
        let chunk_size = len / num_chunks;

        let mut rng = rand::rng();
        let mut res: Vec<usize> = Vec::with_capacity(num_chunks);

        for chunk in curr_vec.chunks_exact(chunk_size) {
            // We already validated the chunks, so unwrap() is fine here
            let selection = *chunk.choose(&mut rng).unwrap();
            res.push(selection);
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

    pub fn get_current(&self) -> impl Iterator<Item = usize> {
        self.current.iter().copied()
    }

    pub fn get_removed(&self) -> impl Iterator<Item = usize> {
        self.removed.iter().copied()
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
            Action::Add(i) => self._add(i),
            Action::Remove(i) => self._remove(i),
        };
    }

    fn undo_action(&mut self, action: Action) {
        match action {
            Action::Add(i) => self._remove(i),
            Action::Remove(i) => self._add(i),
        };
    }

    pub fn path(&self) -> PathBuf {
        SAVES_DIR.join(&self.save_name)
    }
}
