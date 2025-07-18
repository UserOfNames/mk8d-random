use super::course::Course;
use serde::{Deserialize, Serialize};
use std::fmt;

// An action the user takes, e.g. adding or removing a course
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum Action {
    Add(Course),
    Remove(Course),
}

impl fmt::Display for Action {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Action::Add(c) => write!(f, "Add({})", c.name),
            Action::Remove(c) => write!(f, "Remove({})", c.name),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct History {
    past: Vec<Action>,
    future: Vec<Action>,
}

impl History {
    pub fn new() -> Self {
        History {
            past: Vec::new(),
            future: Vec::new(),
        }
    }

    pub fn push(&mut self, action: Action) {
        self.future.clear();
        self.past.push(action);
    }

    pub fn back(&mut self) -> Option<Action> {
        let res = self.past.pop()?;
        self.future.push(res.clone());
        Some(res)
    }

    pub fn forward(&mut self) -> Option<Action> {
        let res = self.future.pop()?;
        self.past.push(res.clone());
        Some(res)
    }

    pub fn has_history(&self) -> bool {
        return !self.past.is_empty();
    }

    pub fn reset(&mut self) {
        self.past.clear();
        self.future.clear();
    }
}

impl fmt::Display for History {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let past_slice = self.past.as_slice();
        let future_slice = self.future.as_slice();

        // Past
        match past_slice {
            [] => write!(f, "None")?,
            [p1] => write!(f, "None <- {}", p1)?,
            [.., p1] => write!(f, "... <- {}", p1)?,
        }

        // Current
        write!(f, " <- Current -> ")?;

        // Future
        match future_slice {
            [] => write!(f, "None")?,
            [p1] => write!(f, "{} -> None", p1)?,
            [.., p1] => write!(f, "{} -> ...", p1)?,
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let hist = History::new();
        assert!(hist.past.is_empty());
        assert!(hist.future.is_empty());
    }

    #[test]
    fn test_push() {
        let mut hist = History::new();
        // Is using the Course constructor here an inter-module test dependency?
        // Perhaps it is. However, I've decided that I don't care.
        hist.push(Action::Add(Course::new(1, 111, "One")));
        assert_eq!(hist.past.len(), 1);
        hist.push(Action::Add(Course::new(2, 112, "Two")));
        assert_eq!(hist.past.len(), 2);
    }

    #[test]
    fn test_has_history() {
        let mut hist = History::new();
        assert!(!hist.has_history());
        hist.push(Action::Add(Course::new(1, 111, "One")));
        hist.push(Action::Add(Course::new(2, 112, "Two")));
        assert!(hist.has_history());
    }

    // The remaining method functionalities are fairly intertwined, so we test them all at once
    #[test]
    fn test_back_forward_push() {
        let course1 = Course::new(1, 111, "One");
        let course2 = Course::new(2, 112, "Two");

        let mut hist = History::new();
        hist.push(Action::Remove(course1.clone()));
        hist.push(Action::Add(course2.clone()));

        let res = hist.back();
        assert_eq!(hist.past.len(), 1);
        assert_eq!(hist.future.len(), 1);

        match res.unwrap() {
            Action::Add(c) => assert_eq!(c, course2),
            Action::Remove(_) => panic!(),
        };

        match &hist.future[0] {
            Action::Add(c) => assert_eq!(*c, course2),
            Action::Remove(_) => panic!(),
        };

        let res = hist.back();
        assert_eq!(hist.past.len(), 0);
        assert_eq!(hist.future.len(), 2);

        match res.unwrap() {
            Action::Add(_) => panic!(),
            Action::Remove(c) => assert_eq!(c, course1),
        };

        match &hist.future[1] {
            Action::Add(_) => panic!(),
            Action::Remove(c) => assert_eq!(*c, course1),
        };

        let res = hist.back();
        match res {
            Some(_) => panic!(),
            None => (),
        }

        let res = hist.forward();
        assert_eq!(hist.past.len(), 1);
        assert_eq!(hist.past.len(), 1);

        match res.unwrap() {
            Action::Add(_) => panic!(),
            Action::Remove(c) => assert_eq!(c, course1),
        };

        match &hist.future[0] {
            Action::Add(c) => assert_eq!(*c, course2),
            Action::Remove(_) => panic!(),
        };

        // Go ahead and test that push() clears future
        hist.push(Action::Add(Course::new(3, 113, "Three")));
        assert_eq!(hist.past.len(), 2);
        assert_eq!(hist.future.len(), 0);
    }
}
