use std::fs::DirEntry;
use std::io;

use mk8d_random::courses::course_list::CourseList;
use ratatui::DefaultTerminal;

#[derive(Debug)]
pub enum Screen {
    Load,          // Loading screen
    Current,       // Displays courses in current
    Removed,       // Displays courses in removed
    SearchCurrent, // Filter current
    SearchRemoved, // Filter removed
    Tiered,        // Tiered sublist
}

#[derive(Debug)]
pub enum Popup {
    Quit, // Quit confirmation
}

#[derive(Debug)]
pub struct Tui {
    pub screen: Screen,
    pub course_list: CourseList,
    pub popup: Option<Popup>,
}

impl Tui {
    pub fn new(saves: Vec<DirEntry>) -> anyhow::Result<Self> {
        if saves.is_empty() {
            return Self::pick_default();
        }

        todo!("TUI constructor");
    }

    fn pick_default() -> anyhow::Result<Self> {
        todo!();
    }

    fn load_save(saves: Vec<DirEntry>) -> anyhow::Result<Self> {
        todo!();
    }

    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        todo!("TUI run");
    }
}
