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

    pub fn render(&mut self) -> Result<(), Error> {
        if !self.should_redraw {
            return Ok(())
        }

        self.render_buffer()?;

        if self.buffer.is_empty() {
            Self::draw_welcome_message()?;
        } 

        self.should_redraw = false;

        Ok(())
    }

    fn render_buffer(&self) -> Result<(), Error> {
        let Size { width, .. } = term::size()?;

        for (idx, line) in self.buffer.lines.iter().enumerate() {
            let truncated_line = line.get(0..width).unwrap_or(line);

            Self::render_line(idx, truncated_line)?;
        }

        Ok(())
    }

    fn render_line(at: usize, line_text: &str) -> Result<(), Error> {
        term::move_caret_to(Position { row: at, col: 0 })?;
        term::clear_line()?;
        term::print(line_text)?;
        Ok(())
    }

    fn draw_welcome_message() -> Result<(), Error> {
        let Size { height, width } = term::size()?;
        let y = height / 2 - 1;

        let x = width / 2 - NAME.len() / 2;
        term::move_caret_to(Position { col: x, row: y })?;
        term::print(NAME)?;

        let x = width / 2 - VERSION.len() / 2;
        term::move_caret_to(Position { col: x, row: y + 1 })?;
        term::print(VERSION)?;
        Ok(())
    }
}

impl Default for View {
    fn default() -> Self {
        Self {
            buffer: Buffer::default(),
            should_redraw: true,
            size: Size::default(),
        }
    }
}
