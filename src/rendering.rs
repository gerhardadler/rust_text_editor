use crate::{cursor::Cursor, text_buffer::TextBuffer, view::View};
use crossterm::{
    cursor,
    style::{self, Stylize},
    terminal, QueueableCommand,
};
use log::debug;
use std::io::{self, Write};

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

    let num_spaces = view.width - 2 - cursor.x.to_string().len() - cursor.y.to_string().len();
    let footer = format!(
        "{: <space$}{cursor_y}:{cursor_x} ",
        "",
        cursor_y = cursor.y,
        cursor_x = cursor.x,
        space = num_spaces
    );

    stdout.queue(cursor::MoveTo(0, view.height as u16))?;
    stdout.queue(style::Print(format!(
        "{}",
        style::PrintStyledContent(footer.reverse())
    )))?;

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
