use crossterm::event::{Event, KeyCode, KeyEvent, KeyModifiers};

use crate::{cursor::Cursor, text_buffer::TextBuffer, view::View};

pub enum StopEventLoop {
    Yes(),
    No(),
}

pub fn event_handler(
    event: &Event,
    text_buffer: &mut TextBuffer,
    cursor: &mut Cursor,
    view: &mut View,
) -> StopEventLoop {
    match event {
        Event::Key(event) => key_handler(&event, text_buffer, cursor, view),
        _ => StopEventLoop::No(),
    }
}

fn key_handler(
    event: &KeyEvent,
    text_buffer: &mut TextBuffer,
    cursor: &mut Cursor,
    view: &mut View,
) -> StopEventLoop {
    match event.code {
        KeyCode::Char(char) => {
            if let KeyModifiers::CONTROL = event.modifiers {
                match char {
                    'z' => text_buffer.undo(cursor),
                    'y' => text_buffer.redo(cursor),
                    _ => (),
                }
            } else {
                if char == ' ' {
                    text_buffer.new_change_frame(cursor.clone());
                }
                let mut line = text_buffer.lines[cursor.y].clone();
                line.insert(cursor.x, char);
                text_buffer.remove(cursor.y);
                text_buffer.insert(cursor.y, line);
                cursor.move_x(1, &text_buffer.lines);
            }
        }
        KeyCode::Backspace => {
            if cursor.x == 0 {
                if cursor.y > 0 {
                    let line = text_buffer.remove(cursor.y);
                    let mut prev_line = text_buffer.remove(cursor.y - 1);
                    let old_prev_line_len = prev_line.len();
                    prev_line.push_str(&line);
                    text_buffer.insert(cursor.y - 1, prev_line);

                    cursor.move_y(-1, &text_buffer.lines);
                    cursor.move_x(old_prev_line_len as isize, &text_buffer.lines);
                };
            } else {
                cursor.move_x(-1, &text_buffer.lines);
                let mut line = text_buffer.lines[cursor.y].clone();
                line.remove(cursor.x);
                text_buffer.remove(cursor.y);
                text_buffer.insert(cursor.y, line);
            }
            text_buffer.new_change_frame(cursor.clone());
        }
        KeyCode::Delete => {
            if cursor.x == text_buffer.lines[cursor.y].len() {
                if cursor.y + 1 < text_buffer.lines.len() {
                    let next_line = text_buffer.remove(cursor.y + 1);
                    let mut line = text_buffer.remove(cursor.y);
                    line.push_str(&next_line);
                    text_buffer.insert(cursor.y, line);
                };
            } else {
                cursor.move_x(-1, &text_buffer.lines);
                let mut line = text_buffer.lines[cursor.y].clone();
                line.remove(cursor.x);
                text_buffer.remove(cursor.y);
                text_buffer.insert(cursor.y, line);
            }
            text_buffer.new_change_frame(cursor.clone());
        }
        KeyCode::Enter => {
            if cursor.x == text_buffer.lines[cursor.y].len() {
                text_buffer.insert(cursor.y + 1, String::new());
            } else {
                let mut current_line = text_buffer.remove(cursor.y);
                let excess: String = current_line.drain(cursor.x..).collect();
                text_buffer.insert(cursor.y, current_line);
                text_buffer.insert(cursor.y + 1, excess);
            }
            cursor.move_y(1, &text_buffer.lines);
            cursor.set_x(0, &text_buffer.lines);
            text_buffer.new_change_frame(cursor.clone());
        }
        KeyCode::Up => {
            if let KeyModifiers::ALT = event.modifiers {
                view.move_v_scroll(-1);
            } else {
                cursor.move_y(-1, &text_buffer.lines)
            }
        }
        KeyCode::Down => {
            if let KeyModifiers::ALT = event.modifiers {
                view.move_v_scroll(1);
            } else {
                cursor.move_y(1, &text_buffer.lines)
            }
        }
        KeyCode::Left => {
            if let KeyModifiers::ALT = event.modifiers {
                view.move_h_scroll(-1);
            } else {
                cursor.move_x(-1, &text_buffer.lines)
            }
        }
        KeyCode::Right => {
            if let KeyModifiers::ALT = event.modifiers {
                view.move_h_scroll(1);
            } else {
                cursor.move_x(1, &text_buffer.lines)
            }
        }
        KeyCode::Esc => return StopEventLoop::Yes(),
        _ => (),
    };
    StopEventLoop::No()
}
