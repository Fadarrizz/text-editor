use crossterm::event::{Event, KeyCode, KeyEvent, read};
mod terminal;
mod view;
use std::panic::{set_hook, take_hook};
use std::{cmp::min, io::Error, path::PathBuf};
use terminal as term;
use terminal::{Position, Size};
use view::View;

use crate::Args;

pub struct Editor {
    should_quit: bool,
    position: Position,
    view: View,
}

impl Editor {
    pub fn new(args: &Args) -> Result<Self, Error> {
        let current_hook = take_hook();
        set_hook(Box::new(move |panic_info| {
            let _ = term::terminate();
            current_hook(panic_info);
        }));

        term::initialize()?;

        let mut editor = Editor {
            should_quit: false,
            position: Position::default(),
            view: View::default(),
        };

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
        loop {
            self.refresh_screen();

            if self.should_quit {
                break;
            }

            match read() {
                Ok(event) => self.evaluate_event(&event),
                Err(err) => {
                    #[cfg(debug_assertions)]
                    {
                        panic!("Could not read event: {err:?}");
                    }
                }
            }
        }
    }

    fn refresh_screen(&mut self) {
        let _ = term::hide_caret();
        let _ = term::move_caret_to(Position::default());
        self.view.render();
        let _ = term::move_caret_to(self.position);
        let _ = term::show_caret();
        let _ = term::execute();
    }

    fn evaluate_event(&mut self, event: &Event) {
        match event {
            Event::Key(KeyEvent { code, .. }) => match code {
                KeyCode::Esc => {
                    self.should_quit = true;
                }
                KeyCode::Char('h' | 'j' | 'k' | 'l') => {
                    self.move_cursor(*code);
                }
                _ => {}
            },
            Event::Resize(width_u16, height_u16) => {
                self.view.resize(Size {
                    width: *width_u16 as usize,
                    height: *height_u16 as usize,
                });
            }
            _ => {}
        }
    }

    fn move_cursor(&mut self, key_code: KeyCode) {
        let Position {
            col: mut x,
            row: mut y,
        } = self.position;

        let Size { width, height } = term::size().unwrap_or_default();

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
    }
}

impl Drop for Editor {
    fn drop(&mut self) {
        let _ = term::terminate();
        if self.should_quit {
            let _ = term::print("Goodbye.\r\n");
        }
    }
}
