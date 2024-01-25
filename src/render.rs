use std::io::{self, Write};

use crossterm::{cursor, style, terminal, QueueableCommand};

use crate::cursor::Cursor;

pub fn render(lines: &Vec<String>, cursor: &Cursor) -> io::Result<()> {
    let mut stdout = io::stdout();
    stdout.queue(cursor::MoveTo(0, 0))?;
    stdout.queue(terminal::Clear(terminal::ClearType::All))?;

    for line in lines {
        stdout.queue(style::Print(format!("{}\r\n", line)))?;
    }
    stdout.queue(cursor::MoveTo(cursor.x as u16, cursor.y as u16))?;
    stdout.queue(cursor::Show)?;

    stdout.flush()?;
    Ok(())
}
