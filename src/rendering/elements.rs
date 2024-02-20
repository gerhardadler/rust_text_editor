use std::io::{self, Stdout};

use crossterm::{
    cursor,
    style::{self, Stylize},
    QueueableCommand,
};

use crate::{cursor::Cursor, view::View};

pub fn render_footer(stdout: &mut Stdout, view: &View, cursor: &Cursor) -> io::Result<()> {
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
    Ok(())
}
