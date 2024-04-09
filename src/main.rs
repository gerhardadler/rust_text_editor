use std::fs::File;
use std::{env, io, panic};

use crossterm::terminal::{EnterAlternateScreen, LeaveAlternateScreen};
use crossterm::{execute, terminal};
use log::{debug, LevelFilter};
use rust_text_editor::{events::event_loop, read_lines, text_buffer::TextBuffer};
use simplelog::{CombinedLogger, Config, WriteLogger};

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = "test.txt";
    let lines = read_lines::read_lines(&file_path).unwrap();

    CombinedLogger::init(vec![WriteLogger::new(
        LevelFilter::Debug,
        Config::default(),
        File::create("test.log").unwrap(),
    )])
    .unwrap();

    debug!("PROGRAM START");

    terminal::enable_raw_mode().expect("Failed to enable raw mode");
    execute!(io::stdout(), EnterAlternateScreen).unwrap();

    let default_panic = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |info| {
        terminal::disable_raw_mode().expect("Failed to disable raw mode");
        execute!(io::stdout(), LeaveAlternateScreen).unwrap();
        default_panic(info);
    }));

    let result = event_loop::event_loop(lines);

    match result {
        Ok(_) => (),
        Err(e) => {
            terminal::disable_raw_mode().expect("Failed to disable raw mode");
            execute!(io::stdout(), LeaveAlternateScreen).unwrap();
            panic!("Error: {:?}", e);
        }
    }
}
