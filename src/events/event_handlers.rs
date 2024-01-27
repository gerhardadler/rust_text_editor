use crossterm::event::{Event, KeyCode};

use crate::{cursor::Cursor, text_buffer::TextBuffer};

pub enum StopEventLoop {
    Yes(),
    No(),
}

pub fn event_handler(
    event: &Event,
    text_buffer: &mut TextBuffer,
    cursor: &mut Cursor,
) -> StopEventLoop {
    match event {
        Event::Key(event) => key_handler(&event.code, text_buffer, cursor),
        _ => StopEventLoop::No(),
    }
}

fn key_handler(
    key_code: &KeyCode,
    text_buffer: &mut TextBuffer,
    cursor: &mut Cursor,
) -> StopEventLoop {
    match key_code {
        KeyCode::Char(char) => {
            let mut line = text_buffer.lines[cursor.y].clone();
            line.insert(cursor.x, *char);
            text_buffer.remove(cursor.y);
            text_buffer.insert(cursor.y, line);
            cursor.move_x(1, &text_buffer.lines);
        }
        // KeyCode::Backspace => {
        //     if cursor.x == 0 {
        //         if cursor.y > 0 {
        //             let line = lines.remove(cursor.y);
        //             if let Some(prev_line) = lines.get_mut(cursor.y - 1) {
        //                 let old_prev_line_len = prev_line.len() as isize;
        //                 prev_line.push_str(&line);
        //                 cursor.move_y(-1, lines);
        //                 cursor.move_x(old_prev_line_len, lines);
        //             }
        //         };
        //     } else {
        //         cursor.move_x(-1, lines);
        //         if let Some(line) = lines.get_mut(cursor.y) {
        //             line.remove(cursor.x);
        //         }
        //     }
        // }
        // KeyCode::Delete => {
        //     let current_line = &lines[cursor.y];
        //     if cursor.x == current_line.len() {
        //         if cursor.y + 1 < lines.len() {
        //             let next_line = lines.remove(cursor.y + 1);
        //             if let Some(line) = lines.get_mut(cursor.y) {
        //                 line.push_str(&next_line);
        //             }
        //         };
        //     } else {
        //         if let Some(line) = lines.get_mut(cursor.y) {
        //             line.remove(cursor.x);
        //         }
        //     }
        // }
        // KeyCode::Enter => {
        //     let current_line = &mut lines[cursor.y];

        //     let excess: String = current_line.drain(cursor.x..).collect();
        //     lines.insert(cursor.y + 1, excess);

        //     cursor.move_y(1, lines);
        //     cursor.set_x(0, lines);
        // }
        KeyCode::Up => cursor.move_y(-1, &text_buffer.lines),
        KeyCode::Down => cursor.move_y(1, &text_buffer.lines),
        KeyCode::Left => cursor.move_x(-1, &text_buffer.lines),
        KeyCode::Right => cursor.move_x(1, &text_buffer.lines),
        KeyCode::Esc => return StopEventLoop::Yes(),
        _ => (),
    };
    StopEventLoop::No()
}
