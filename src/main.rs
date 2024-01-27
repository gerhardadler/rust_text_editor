use std::env;

use crossterm::terminal;
use rust_text_editor::{events::event_loop, read_lines, text_buffer::TextBuffer};

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = "test.txt";
    let lines = read_lines::read_lines(&file_path).unwrap();
    let mut text_buffer = TextBuffer::new(lines);

    terminal::enable_raw_mode().expect("Failed to enable raw mode");
    event_loop::event_loop(&mut text_buffer).unwrap();
    terminal::disable_raw_mode().expect("Failed to disable raw mode");
}
