use std::fmt;

use serde::{Deserialize, Serialize};

use super::course::Course;

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
