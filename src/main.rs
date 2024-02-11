use std::env;
use std::fs::File;

use crossterm::terminal;
use log::{debug, LevelFilter};
use rust_text_editor::{events::event_loop, read_lines, text_buffer::TextBuffer};
use simplelog::{CombinedLogger, Config, WriteLogger};

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = "test.txt";
    let lines = read_lines::read_lines(&file_path).unwrap();
    let mut text_buffer = TextBuffer::new(lines);

    CombinedLogger::init(vec![WriteLogger::new(
        LevelFilter::Debug,
        Config::default(),
        File::create("test.log").unwrap(),
    )])
    .unwrap();

    debug!("PROGRAM START");

    terminal::enable_raw_mode().expect("Failed to enable raw mode");
    event_loop::event_loop(&mut text_buffer).unwrap();
    terminal::disable_raw_mode().expect("Failed to disable raw mode");
}
