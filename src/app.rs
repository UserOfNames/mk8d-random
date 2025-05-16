use ratatui::{
    DefaultTerminal, Frame,
    backend::Backend,
    buffer::Buffer,
    crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind},
    layout::Rect,
    widgets::Widget,
};
use std::{io, path::PathBuf};

use crate::courses::course_list::CourseList;

#[derive(Debug)]
pub enum CurrentScreen {
    Load,      // Loading screen
    In,        // Displays courses in current
    Out,       // Displays courses in removed
    SearchIn,  // Filter current
    SearchOut, // Filter removed
    Tiered,    // Tiered sublist
}

#[derive(Debug)]
pub struct App {
    pub current_screen: CurrentScreen,
    pub course_list: CourseList,
    exit: bool,
}

impl App {
    pub fn new() -> Self {
        App {
            course_list: CourseList::new("PLACEHOLDER"),
            current_screen: CurrentScreen::Load,
            exit: false,
        }
    }

    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        while !self.exit {
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

    fn handle_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('q') => self.exit(),
            KeyCode::Char('L') => self.load_screen(),
            KeyCode::Char('i') => self.in_screen(),
            KeyCode::Char('o') => self.out_screen(),
            KeyCode::Char('s') => self.search(),
            KeyCode::Char('t') => self.tier(),
            _ => {}
        }
    }

    fn draw(&self, frame: &mut Frame) {
        todo!("Draw logic")
    }

    fn exit(&mut self) {
        todo!("Exit confirmation popup and save logic");
        self.exit = true;
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
