use crossterm::event::{Event, KeyCode, KeyEvent, read};
mod terminal;
mod view;
use std::{cmp::min, io::Error, path::PathBuf};
use terminal as term;
use terminal::{Position, Size};
use view::View;

use crate::Args;

#[derive(Default)]
pub struct Editor {
    should_quit: bool,
    position: Position,
    view: View,
}

impl Editor {
    pub fn new(args: &Args) -> Result<Self, Error> {
        let mut editor = Editor::default();

        if let Some(file) = args.files.get(1) {
            editor.open(file)?;
        }

        Ok(editor)
    }

    pub fn open(&mut self, path: &PathBuf) -> Result<(), Error> {
        let view = View::load(path)?;
        self.view = view;
        Ok(())
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
            self.evaluate_event(&event)?;
            if self.should_quit {
                break;
            }
        }
        Ok(())
    }

    fn refresh_screen(&mut self) -> Result<(), Error> {
        term::hide_caret()?;
        term::move_caret_to(Position::default())?;
        if self.should_quit {
            term::clear_screen()?;
            term::print("Goodbye.\r\n")?;
        } else {
            self.view.render()?;
            term::move_caret_to(self.position)?;
        }
        term::show_caret()?;
        term::execute()?;
        Ok(())
    }

    fn move_cursor(&mut self, key_code: KeyCode) -> Result<(), Error> {
        let Position {
            col: mut x,
            row: mut y,
        } = self.position;
        let Size { width, height } = term::size()?;

        match key_code {
            KeyCode::Char('h') => {
                x = x.saturating_sub(1);
            }
            KeyCode::Char('j') => {
                y = min(height, y.saturating_add(1));
            }
            KeyCode::Char('k') => {
                y = y.saturating_sub(1);
            }
            KeyCode::Char('l') => {
                x = min(width, x.saturating_add(1));
            }
            _ => (),
        }
        self.position = Position { col: x, row: y };

        Ok(())
    }

    fn evaluate_event(&mut self, event: &Event) -> Result<(), Error> {
        match event {
            Event::Key(KeyEvent { code, .. }) => {
                match code {
                    KeyCode::Esc => {
                        self.should_quit = true;
                    }
                    KeyCode::Char('h' | 'j' | 'k' | 'l') => {
                        self.move_cursor(*code)?;
                    }
                    _ => {},
                }
            },
            Event::Resize(width_u16, height_u16) => {
                self.view.resize(Size {
                    width: *width_u16 as usize,
                    height: *height_u16 as usize,
                });
            },
            _ => {},
        }
        Ok(())
    }
}
