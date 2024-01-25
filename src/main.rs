use std::env;

use crossterm::terminal;
use rust_text_editor::{events::event_loop, read_lines};

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    let mut lines = read_lines::read_lines(&file_path).unwrap();

    terminal::enable_raw_mode().expect("Failed to enable raw mode");
    event_loop::event_loop(&mut lines).unwrap();
    terminal::disable_raw_mode().expect("Failed to disable raw mode");
}
