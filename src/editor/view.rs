use super::terminal as term;
use crate::editor::{Size, terminal::Position, view::buffer::Buffer};
use std::{io::Error, path::PathBuf};
mod buffer;

const NAME: &str = env!("CARGO_PKG_NAME");
const VERSION: &str = env!("CARGO_PKG_VERSION");

pub struct View {
    buffer: Buffer,
    should_redraw: bool,
    size: Size,
}

impl View {
    pub fn load(path: &PathBuf) -> Result<Self, Error> {
        let buffer = Buffer::load(path)?;

        let view = Self { buffer, ..Default::default() };

        Ok(view)
    }

    pub fn resize(&mut self, to: Size) {
        self.size = to;
        self.should_redraw = true;
    }

    pub fn render(&mut self) {
        if !self.should_redraw {
            return;
        }

        let Size { width, height } = self.size;
        if width == 0 || height == 0 {
            return;
        }

        if self.buffer.is_empty() {
            self.draw_welcome_message();

        } else {
            for (idx, line) in self.buffer.lines.iter().enumerate() {
                let truncated_line = line.get(0..width).unwrap_or(line);

                Self::render_line(idx, truncated_line);
            }
        } 

        self.should_redraw = false;
    }

    fn render_line(at: usize, line_text: &str) {
        let result = term::print_line(at, line_text);

        debug_assert!(result.is_ok(), "Failed to render line");
    }

    fn draw_welcome_message(&self) {
        let Size { height, width } = self.size;
        let y = height / 2 - 1;

        let x = width / 2 - NAME.len() / 2;
        let _ = term::move_caret_to(Position { col: x, row: y });
        let _ = term::print(NAME);

        let x = width / 2 - VERSION.len() / 2;
        let _ = term::move_caret_to(Position { col: x, row: y + 1 });
        let _ = term::print(VERSION);
    }
}

impl Default for View {
    fn default() -> Self {
        Self {
            buffer: Buffer::default(),
            should_redraw: true,
            size: term::size().unwrap_or_default(),
        }
    }
}
