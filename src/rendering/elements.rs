fn render_footer() {
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
}
