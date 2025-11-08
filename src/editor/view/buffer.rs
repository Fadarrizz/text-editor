use std::{io::Error, path::PathBuf};

#[derive(Default)]
pub struct Buffer {
    pub lines: Vec<String>,
}

impl Buffer {
    pub fn load(path: &PathBuf) -> Result<Self, Error> {
        let contents = std::fs::read_to_string(path)?;

        let lines: Vec<String> = contents
            .lines()
            .map(|s: &str| s.to_string())
            .collect();

        Ok(Self { lines })
    }

    pub fn is_empty(&self) -> bool {
        self.lines.is_empty()
    }
}
