use std::io;

use crossterm::event;

use crate::{cursor::Cursor, render};

use super::event_handlers::{event_handler, StopEventLoop};

pub fn event_loop(lines: &mut Vec<String>) -> io::Result<()> {
    let mut cursor = Cursor::new(0, 0);
    loop {
        let stop_event_loop = event_handler(&event::read()?, lines, &mut cursor);
        if let StopEventLoop::Yes() = stop_event_loop {
            break;
        }
        render::render(&lines, &cursor).unwrap();
    }
    Ok(())
}
