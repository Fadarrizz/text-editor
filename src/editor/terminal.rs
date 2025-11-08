use crossterm::terminal::{Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode};
use crossterm::{
    Command,
    cursor::{Hide, MoveTo, Show},
    queue,
    style::Print,
};
use std::io::{Error, Write, stdout};

#[derive(Copy, Clone, Default)]
pub struct Size {
    pub width: usize,
    pub height: usize,
}

#[derive(Copy, Clone, Default)]
pub struct Position {
    pub col: usize,
    pub row: usize,
}

pub fn initialize() -> Result<(), Error> {
    enable_raw_mode()?;
    enter_alternate_screen()?;
    clear_screen()?;
    Ok(())
}

pub fn terminate() -> Result<(), Error> {
    disable_raw_mode()?;
    leave_alternate_screen()?;
    Ok(())
}

pub fn enter_alternate_screen() -> Result<(), Error> {
    queue_command(EnterAlternateScreen)?;
    Ok(())
}

pub fn leave_alternate_screen() -> Result<(), Error> {
    queue_command(LeaveAlternateScreen)?;
    Ok(())
}

pub fn clear_screen() -> Result<(), Error> {
    queue_command(Clear(ClearType::All))?;
    Ok(())
}

pub fn clear_line() -> Result<(), Error> {
    queue_command(Clear(ClearType::CurrentLine))?;
    Ok(())
}

/// Moves the caret to the given Position.
/// # Arguments
/// * `Position` - the `Position` to move the caret to. Will be truncated to `u16::MAX` if bigger.
pub fn move_caret_to(p: Position) -> Result<(), Error> {
    #[allow(clippy::as_conversions, clippy::cast_possible_truncation)]
    queue_command(MoveTo(p.col as u16, p.row as u16))?;
    Ok(())
}

pub fn hide_caret() -> Result<(), Error> {
    queue_command(Hide)?;
    Ok(())
}

pub fn show_caret() -> Result<(), Error> {
    queue_command(Show)?;
    Ok(())
}

pub fn print_line(row: usize, line_text: &str) -> Result<(), Error> {
    move_caret_to(Position { row, col: 0 })?;
    clear_line()?;
    print(line_text)?;
    Ok(())
}

pub fn size() -> Result<Size, Error> {
    let (width, height) = crossterm::terminal::size()?;
    Ok(Size {
        width: width as usize,
        height: height as usize,
    })
}

pub fn print(string: &str) -> Result<(), Error> {
    queue_command(Print(string))?;
    Ok(())
}

fn queue_command(command: impl Command) -> Result<(), Error> {
    queue!(stdout(), command)?;
    Ok(())
}

pub fn execute() -> Result<(), Error> {
    stdout().flush()?;
    Ok(())
}
