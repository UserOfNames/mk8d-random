//! Module defining the main `CourseList` struct which holds the list of courses in the game, that
//! list's save name, which courses are active, and a history of actions.

#![allow(clippy::result_unit_err)]

use std::collections::BTreeSet;
use std::fs::{self, File};
use std::io::{self, Write};
use std::path::PathBuf;

use rand::seq::{IndexedRandom, IteratorRandom};
use serde::{Deserialize, Serialize};

use crate::SAVES_DIR;

use super::course::Course;
use super::history::Action;
use super::history::History;

/// Main course list struct.
#[derive(Serialize, Deserialize, Debug)]
pub struct CourseList {
    /// List of all courses in the game. This should generally be set once and then left alone.
    /// Courses should be listed by rank in descending order.
    pub courses: Vec<Course>,
    /// Name of the save file as found in `SAVES_DIR`. Should be a relative path ending in .json.
    /// For example, if there is a save `SAVES_DIR/save.json`, this field would be "save.json".
    pub save_name: PathBuf,
    current: BTreeSet<usize>,
    history: History,
}

impl CourseList {
    /// Create an empty `CourseList` with the given `save_name`.
    pub fn new(save_name: impl Into<PathBuf>) -> Self {
        CourseList {
            courses: Vec::new(),
            save_name: save_name.into(),
            current: BTreeSet::new(),
            history: History::default(),
        }
    }

    /// Restore an existing, saved `CourseList` in the `SAVES_DIR` with the filename `save_name`.
    /// `save_name` should be a `*.json` path relative to `SAVES_DIR`. For example, if there is a
    /// save named `my_save`, you can load it by calling this function with `my_save.json`.
    ///
    /// # Errors
    /// - The given `save_name` does not exist in `SAVES_DIR`.
    /// - The save file cannot be deserialized into a valid `CourseList`.
    pub fn restore_save(save_name: impl Into<PathBuf>) -> io::Result<Self> {
        let save_path = SAVES_DIR.join(save_name.into());
        let data = fs::read_to_string(save_path)?;
        Ok(serde_json::from_str(&data)?)
    }

    /// Serialize and save the list to `SAVES_DIR/self.save_name`.
    ///
    /// # Errors
    /// - If the `CourseList` cannot be serialized to a JSON string.
    /// - If `SAVES_DIR/self.save_name` cannot be opened for writing.
    /// - If `SAVES_DIR/self.save_name` is opened for writing, but writing failed.
    pub fn dump_list(&self) -> io::Result<()> {
        let path = self.save_path();

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

    /// Add a course index back into the list.
    pub fn add(&mut self, course_i: usize) {
        self.inner_add(course_i);
        self.history.push(Action::Add(course_i));
    }

    fn inner_add(&mut self, course_i: usize) {
        self.current.insert(course_i);
    }

    /// Remove an active course index from the list.
    pub fn remove(&mut self, course_i: usize) {
        self.inner_remove(course_i);
        self.history.push(Action::Remove(course_i));
    }

    fn inner_remove(&mut self, course_i: usize) {
        self.current.remove(&course_i);
    }

    /// Search the list of active courses by their names. `searched` is case-insensitive.
    pub fn search_current(&self, searched: &str) -> impl Iterator<Item = usize> {
        let key = searched.to_lowercase();
        self.current
            .iter()
            .copied()
            .filter(move |&i| self.courses[i].name.to_lowercase().contains(&key))
    }

    /// Search the list of removed courses by their names. `searched` is case-insensitive.
    pub fn search_removed(&self, searched: &str) -> impl Iterator<Item = usize> {
        let key = searched.to_lowercase();
        self.get_removed()
            .filter(move |&i| self.courses[i].name.to_lowercase().contains(&key))
    }

    /// Get a random active course.
    pub fn get_random(&self) -> Option<usize> {
        if self.current.is_empty() {
            return None;
        }

        self.current.iter().choose(&mut rand::rng()).copied()
    }

    /// Split the list into N chunks, then grab a random course from each chunk.
    ///
    /// # Errors
    /// - If the current number of active courses cannot be evenly divded by the given number of
    ///   chunks.
    pub fn get_random_by_chunks(
        &self,
        num_chunks: usize,
    ) -> Result<impl Iterator<Item = usize>, ()> {
        let curr_vec: Vec<usize> = self.current.iter().copied().collect();
        let len = self.current.len();

        if !len.is_multiple_of(num_chunks) {
            return Err(());
        }
        let chunk_size = len / num_chunks;

        let mut rng = rand::rng();
        let mut res: Vec<usize> = Vec::with_capacity(num_chunks);

        for chunk in curr_vec.chunks_exact(chunk_size) {
            // We already validated the chunks, so unwrap() is fine here
            #[allow(clippy::missing_panics_doc)]
            let selection = *chunk.choose(&mut rng).unwrap();
            res.push(selection);
        }

        Ok(res.into_iter())
    }

    /// Make all courses active and clear all history.
    pub fn reset(&mut self) {
        self.current.extend(self.get_removed());
        self.history.reset();
    }

    /// Get a view of the action history.
    #[inline]
    pub fn get_history(&self) -> &History {
        &self.history
    }

    /// Get a view of active courses.
    #[inline]
    pub fn get_current(&self) -> impl Iterator<Item = usize> {
        self.current.iter().copied()
    }

    /// Get a view of removed courses.
    pub fn get_removed(&self) -> impl Iterator<Item = usize> + use<> {
        let current = self.current.clone();

        (0..self.courses.len())
            .filter(move |x| !current.contains(x))
    }

    /// Undo the most recent action.
    ///
    /// # Errors
    /// - If there are no actions to roll back.
    pub fn roll_back(&mut self) -> Result<(), ()> {
        let action: Action = self.history.back().ok_or(())?;
        self.undo_action(action);
        Ok(())
    }

    /// Redo the most recently undone action.
    ///
    /// # Errors
    /// - If there are no undone actions.
    pub fn roll_forward(&mut self) -> Result<(), ()> {
        let action: Action = self.history.forward().ok_or(())?;
        self.apply_action(action);
        Ok(())
    }

    fn apply_action(&mut self, action: Action) {
        match action {
            Action::Add(i) => self.inner_add(i),
            Action::Remove(i) => self.inner_remove(i),
        }
    }

    fn undo_action(&mut self, action: Action) {
        match action {
            Action::Add(i) => self.inner_remove(i),
            Action::Remove(i) => self.inner_add(i),
        }
    }

    /// Construct the path `SAVES_DIR/self.save_name`.
    pub fn save_path(&self) -> PathBuf {
        SAVES_DIR.join(&self.save_name)
    }
}
