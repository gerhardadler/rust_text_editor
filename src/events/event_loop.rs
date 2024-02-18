use super::event_handlers::{event_handler, StopEventLoop};
use crate::{cursor::Cursor, rendering, text_buffer::TextBuffer, view::View};
use crossterm::event;
use std::io;

pub fn event_loop(text_buffer: &mut TextBuffer) -> io::Result<()> {
    let mut cursor = Cursor::new(0, 0);
    let mut view = View {
        width: 10,
        height: 10,
        v_scroll: 0,
        h_scroll: 0,
    };
    loop {
        rendering::render(&text_buffer, &cursor, &view).unwrap();
        let stop_event_loop = event_handler(&event::read()?, text_buffer, &mut cursor, &mut view);
        if let StopEventLoop::Yes() = stop_event_loop {
            break;
        }
    }
    Ok(())
}
