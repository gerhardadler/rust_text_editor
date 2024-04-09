use super::event_handlers::{event_handler, StopEventLoop};
use crate::{cursor::Cursor, rendering, text_buffer::TextBuffer, view::View};
use crossterm::{event, terminal};
use std::io;

pub fn event_loop(lines: Vec<String>) -> io::Result<()> {
    let mut text_buffer = TextBuffer::new(lines);
    let mut cursor = Cursor::new(0, 0);
    let terminal_size = terminal::size()?;
    let mut view = View {
        width: terminal_size.0 as usize,
        height: terminal_size.0 as usize,
        v_scroll: 0,
        h_scroll: 0,
    };
    loop {
        rendering::render(&text_buffer, &cursor, &view).unwrap();
        let stop_event_loop =
            event_handler(&event::read()?, &mut text_buffer, &mut cursor, &mut view);
        if let StopEventLoop::Yes() = stop_event_loop {
            break;
        }
    }
    Ok(())
}
