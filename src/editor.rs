use crossterm::event::{Event, KeyEvent, KeyModifiers, read, Event::Key, KeyCode::Char};
mod terminal;
use terminal as term;
use std::io::Error;

use crate::editor::terminal::Size;

pub struct Editor {
    should_quit: bool,
}

impl Editor {
    pub const fn default() -> Self {
        Self { should_quit: false }
    }

    pub fn run(&mut self) {
        term::initialize().unwrap();
        let result = self.repl();
        term::terminate().unwrap();
        result.unwrap();
    }

    fn repl(&mut self) -> Result<(), Error> {
        loop {
            self.refresh_screen()?;
            let event = read()?;
            self.evaluate_event(&event);
            if self.should_quit {
                break;
            }
        }
        Ok(())
    }

    fn evaluate_event(&mut self, event: &Event) {
        if let Key(KeyEvent { code: Char('q'), modifiers: KeyModifiers::CONTROL, .. }) = event {
            self.should_quit = true;
        }
    }

    fn refresh_screen(&self) -> Result<(), Error> {
        term::hide_cursor()?;
        if self.should_quit {
            term::clear_screen()?;
            term::print("Goodbye.\r\n")?;
        } else {
            Self::draw_rows()?;
            term::move_cursor_to(terminal::Position{x:0, y:0})?;
        }
        term::show_cursor()?;
        term::execute()?;
        Ok(())
    }

    fn draw_rows() -> Result<(), Error> {
        let Size{height, ..} = term::size()?;
        for row in 0..height {
            term::clear_line()?;
            term::print("~")?;
            if row + 1 < height {
                term::print("\r\n")?;
            }
        }
        Ok(())
    }
}
