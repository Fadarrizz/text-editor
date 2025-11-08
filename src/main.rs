#![warn(clippy::all, clippy::pedantic, clippy::print_stdout)]
mod editor;
use std::path::{Path, PathBuf};
use editor::Editor;

#[derive(Debug)]
struct Args {
    files: Vec<PathBuf>,
}

impl Args {
    pub fn new() -> Args {
        let mut args = Self {
            files: Vec::new()
        };

        for arg in std::env::args().collect::<Vec<String>>() {
            let path = Path::new(&arg);
            args.files.push(path.to_path_buf());
        }

        args
    }
}

fn main() -> Result<(), anyhow::Error> {
    let args: Args = Args::new();

    let mut editor = Editor::new(&args)?;
    
    editor.run();

    Ok(())
}
