use std::io;

use crossterm::event;

use crate::{cursor::Cursor, render, text_buffer::TextBuffer};

use super::event_handlers::{event_handler, StopEventLoop};

pub fn event_loop(text_buffer: &mut TextBuffer) -> io::Result<()> {
    let mut cursor = Cursor::new(0, 0);
    loop {
        let stop_event_loop = event_handler(&event::read()?, text_buffer, &mut cursor);
        if let StopEventLoop::Yes() = stop_event_loop {
            break;
        }
        render::render(&text_buffer, &cursor).unwrap();
    }
    Ok(())
}
