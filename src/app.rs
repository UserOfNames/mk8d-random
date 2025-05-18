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
    Load,      // Loading screen
    In,        // Displays courses in current
    Out,       // Displays courses in removed
    SearchIn,  // Filter current
    SearchOut, // Filter removed
    Tiered,    // Tiered sublist
    Quitting,  // Quit confirmation screen
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
            CurrentScreen::In => self.handle_in(key),
            CurrentScreen::Out => self.handle_out(key),
            CurrentScreen::SearchIn => self.handle_search_in(key),
            CurrentScreen::SearchOut => self.handle_search_out(key),
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

    fn draw(&self, frame: &mut Frame) {
        todo!("Draw logic")
    }

    fn quit(&mut self) {
        todo!("Exit confirmation popup and save logic");
    }

    fn load_screen(&mut self) {
        self.current_screen = CurrentScreen::Load;
    }

    fn in_screen(&mut self) {
        self.current_screen = CurrentScreen::In;
    }

    fn out_screen(&mut self) {
        self.current_screen = CurrentScreen::Out;
    }

    fn search(&mut self) {
        todo!("Search")
    }

    fn tier(&mut self) {
        todo!("Tier")
    }
}
