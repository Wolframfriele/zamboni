#![warn(clippy::all, clippy::pedantic)]
mod editor;
use std::io;

use editor::Editor;

fn main() -> io::Result<()> {
    Editor::default().run()?;
    Ok(())
}
