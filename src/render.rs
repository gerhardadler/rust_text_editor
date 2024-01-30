use std::io::{self, Write};

use crossterm::{cursor, style, terminal, QueueableCommand};

use crate::{cursor::Cursor, text_buffer::TextBuffer, view::View};

pub fn render(text_buffer: &TextBuffer, cursor: &Cursor, view: &View) -> io::Result<()> {
    let mut stdout = io::stdout();
    stdout.queue(cursor::MoveTo(0, 0))?;
    stdout.queue(terminal::Clear(terminal::ClearType::All))?;

    for line in text_buffer
        .lines
        .iter()
        .skip(view.v_scroll)
        .take(view.height)
    {
        let capped_line = line
            .chars()
            .skip(view.h_scroll)
            .take(view.width)
            .collect::<String>();
        stdout.queue(style::Print(format!("{}\r\n", capped_line)))?;
    }
    stdout.queue(cursor::MoveTo(
        (cursor.x - view.h_scroll) as u16,
        (cursor.y - view.v_scroll) as u16,
    ))?;
    stdout.queue(cursor::Show)?;

    stdout.flush()?;
    Ok(())
}
