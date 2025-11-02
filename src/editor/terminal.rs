use std::{fmt::Display, io::{Error, Write, stdout}};
use crossterm::{cursor::{Hide, MoveTo, Show}, queue, style::Print};
use crossterm::terminal::{Clear, ClearType, disable_raw_mode, enable_raw_mode};

#[derive(Copy, Clone)]
pub struct Size {
    pub width: u16,
    pub height: u16,
}

#[derive(Copy, Clone)]
pub struct Position {
    pub x: u16,
    pub y: u16,
}

pub fn initialize() -> Result<(), Error> {
    enable_raw_mode()?;
    clear_screen()?;
    move_cursor_to(Position { x: 0, y: 0 })?;
    Ok(())
}

pub fn terminate() -> Result<(), Error> {
    disable_raw_mode()?;
    Ok(())
}

pub fn clear_screen() -> Result<(), Error> {
    queue!(stdout(), Clear(ClearType::All))?;
    Ok(())
}

pub fn clear_line() -> Result<(), Error> {
    queue!(stdout(), Clear(ClearType::CurrentLine))?;
    Ok(())
}

pub fn move_cursor_to(p: Position) -> Result<(), Error> {
    queue!(stdout(), MoveTo(p.x, p.y))?;
    Ok(())
}

pub fn hide_cursor() -> Result<(), Error> {
    queue!(stdout(), Hide)?;
    Ok(())
}

pub fn show_cursor() -> Result<(), Error> {
    queue!(stdout(), Show)?;
    Ok(())
}

pub fn size() -> Result<Size, Error> {
    let (width, height) = crossterm::terminal::size()?;
    Ok(Size { width, height })
}

pub fn print<T: Display>(arg: T) -> Result<(), Error> {
    queue!(stdout(), Print(arg))?;
    Ok(())
}

pub fn execute() -> Result<(), Error> {
    stdout().flush()?;
    Ok(())
}
