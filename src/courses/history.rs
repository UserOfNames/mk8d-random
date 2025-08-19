use serde::{Deserialize, Serialize};

use super::course::Course;

// An action the user takes, e.g. adding or removing a course
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq)]
pub enum Action {
    Add(usize),
    Remove(usize),
}

impl Action {
    fn to_string(self, courses: &[Course]) -> String {
        match self {
            Self::Add(i) => format!("Add({})", courses[i].name),
            Self::Remove(i) => format!("Remove({})", courses[i].name),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct History {
    pub past: Vec<Action>,
    pub future: Vec<Action>,
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
        self.future.push(res);
        Some(res)
    }

    pub fn forward(&mut self) -> Option<Action> {
        let res = self.future.pop()?;
        self.past.push(res);
        Some(res)
    }

    pub fn reset(&mut self) {
        self.past.clear();
        self.future.clear();
    }

    pub fn to_string(&self, courses: &[Course]) -> String {
        let past_slice = self.past.as_slice();
        let future_slice = self.future.as_slice();
        let mut res = String::new();

        match past_slice {
            [] => res.push_str("None"),
            [p1] => res.push_str(format!("None <- {}", p1.to_string(courses)).as_str()),
            [.., p1] => res.push_str(format!("... <- {}", p1.to_string(courses)).as_str()),
        }

        res.push_str(" <- Current -> ");

        match future_slice {
            [] => res.push_str("None"),
            [p1] => res.push_str(format!("{} -> None", p1.to_string(courses)).as_str()),
            [.., p1] => res.push_str(format!("{} -> ...", p1.to_string(courses)).as_str()),
        }

        res
    }
}
