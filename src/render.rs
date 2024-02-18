use std::io::{self, Write};

use crossterm::{cursor, style, terminal, QueueableCommand};
use log::debug;

use crate::{cursor::Cursor, text_buffer::TextBuffer, view::View};

pub fn render(text_buffer: &TextBuffer, cursor: &Cursor, view: &View) -> io::Result<()> {
    let mut stdout = io::stdout();
    stdout.queue(cursor::MoveTo(0, 0))?;
    stdout.queue(terminal::Clear(terminal::ClearType::All))?;

    for (i, line) in text_buffer
        .lines
        .iter()
        .skip(view.v_scroll)
        .take(view.height)
        .enumerate()
    {
        let capped_line = line
            .chars()
            .skip(view.h_scroll)
            .take(view.width)
            .collect::<String>();
        stdout.queue(cursor::MoveTo(0, i as u16))?;
        stdout.queue(style::Print(format!("{}", capped_line)))?;
    }

    if let Some(coordinate) = cursor.get_render_position(view) {
        stdout.queue(cursor::MoveTo(coordinate.x as u16, coordinate.y as u16))?;
        stdout.queue(cursor::Show)?;
    } else {
        stdout.queue(cursor::Hide)?;
    }

    stdout.flush()?;
    debug!(
        "view.width: {}, view.height: {}, view.h_scroll: {}, view.v_scroll: {}",
        view.width, view.height, view.h_scroll, view.v_scroll
    );

    Ok(())
}
