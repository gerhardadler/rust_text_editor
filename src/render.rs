use std::io::{self, Write};

use crossterm::{cursor, style, terminal, QueueableCommand};

use crate::{cursor::Cursor, text_buffer::TextBuffer};

pub fn render(text_buffer: &TextBuffer, cursor: &Cursor) -> io::Result<()> {
    let mut stdout = io::stdout();
    stdout.queue(cursor::MoveTo(0, 0))?;
    stdout.queue(terminal::Clear(terminal::ClearType::All))?;

    for line in &text_buffer.lines {
        stdout.queue(style::Print(format!("{}\r\n", line)))?;
    }
    stdout.queue(cursor::MoveTo(cursor.x as u16, cursor.y as u16))?;
    stdout.queue(cursor::Show)?;

    stdout.flush()?;
    Ok(())
}
