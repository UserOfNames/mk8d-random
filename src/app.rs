use crate::courses::course_list::CourseList;

use ratatui::{
    DefaultTerminal, Frame,
    backend::Backend,
    buffer::Buffer,
    crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind},
    layout::Rect,
    widgets::Widget,
};

use std::{io, path::PathBuf};

#[derive(Debug)]
pub enum CurrentScreen {
    Load,           // Loading screen
    Current,        // Displays courses in current
    Removed,        // Displays courses in removed
    SearchCurrent,  // Filter current
    SearchRemoved,  // Filter removed
    Tiered,         // Tiered sublist
    Quitting,       // Quit confirmation screen
}

#[derive(Debug)]
pub struct App {
    pub current_screen: CurrentScreen,
    pub course_list: CourseList,
    quit: bool,
}

impl App {
    pub fn new() -> Self {
        App {
            course_list: CourseList::new(""),
            current_screen: CurrentScreen::Load,
            quit: false,
        }
    }

    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        while !self.quit {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn draw(&self, frame: &mut Frame) {}

    fn handle_events(&mut self) -> io::Result<()> {
        match event::read()? {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_event(key_event)
            }

            _ => {}
        }

        Ok(())
    }

    fn handle_key_event(&mut self, key: KeyEvent) {
        match self.current_screen {
            CurrentScreen::Load => self.handle_load(key),
            CurrentScreen::Current => self.handle_in(key),
            CurrentScreen::Removed => self.handle_out(key),
            CurrentScreen::SearchCurrent => self.handle_search_in(key),
            CurrentScreen::SearchRemoved => self.handle_search_out(key),
            CurrentScreen::Tiered => self.handle_tiered(key),
            CurrentScreen::Quitting => self.handle_quitting(key),
        }
    }

    fn handle_load(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Char('q') => self.quit(),
            _ => {}
        }
    }

    fn handle_in(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Char('q') => self.quit(),
            _ => {}
        }
    }

    fn handle_out(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Char('q') => self.quit(),
            _ => {}
        }
    }

    fn handle_search_in(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Char('q') => self.quit(),
            _ => {}
        }
    }

    fn handle_search_out(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Char('q') => self.quit(),
            _ => {}
        }
    }

    fn handle_tiered(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Char('q') => self.quit(),
            _ => {}
        }
    }

    fn handle_quitting(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Char('y') => self.quit(),
            _ => {}
        }
    }

    fn quit(&mut self) {
        todo!("Exit confirmation popup and save logic");
    }

    fn load_screen(&mut self) {
        self.current_screen = CurrentScreen::Load;
    }

    fn in_screen(&mut self) {
        self.current_screen = CurrentScreen::Current;
    }

    fn out_screen(&mut self) {
        self.current_screen = CurrentScreen::Removed;
    }

    fn search(&mut self) {
        todo!("Search")
    }

    fn tier(&mut self) {
        todo!("Tier")
    }
}
