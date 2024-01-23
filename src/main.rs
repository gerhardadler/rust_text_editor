use std::{
    env,
    fs::File,
    io::{self, stdout, BufRead, BufReader},
};

use crossterm::{
    cursor,
    event::{read, Event, KeyCode},
    style,
    terminal::{self, disable_raw_mode, enable_raw_mode},
    QueueableCommand,
};

struct Cursor {
    x: usize,
    y: usize,
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    let mut lines = read_lines(&file_path).unwrap();

    enable_raw_mode().expect("Failed to enable raw mode");
    event_loop(&mut lines).unwrap();
    disable_raw_mode().expect("Failed to disable raw mode");
}

fn render(lines: &Vec<String>) -> io::Result<()> {
    let mut stdout = stdout();
    stdout.queue(cursor::MoveTo(0, 0))?;
    stdout.queue(terminal::Clear(terminal::ClearType::All))?;

    for line in lines {
        stdout.queue(style::Print(format!("{}\r\n", line)))?;
    }
    Ok(())
}

fn read_lines(file_path: &str) -> io::Result<Vec<String>> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    // Iterate over lines and collect them into a vector of strings
    let lines: Vec<String> = reader.lines().map(|line| line.unwrap()).collect();
    Ok(lines)
}

fn event_loop(lines: &mut Vec<String>) -> io::Result<()> {
    let mut cursor = Cursor { x: 0, y: 0 };
    loop {
        match read()? {
            Event::FocusGained => println!("FocusGained"),
            Event::FocusLost => println!("FocusLost"),
            Event::Key(event) => match event.code {
                KeyCode::Char(char) => match lines.get_mut(cursor.y) {
                    Some(line) => line.push(char),
                    None => (),
                },
                KeyCode::Up => cursor.y -= 1,
                KeyCode::Down => cursor.y += 1,
                KeyCode::Left => cursor.x -= 1,
                KeyCode::Right => cursor.x += 1,
                _ => (),
            },
            Event::Mouse(event) => println!("{:?}", event),
            Event::Paste(data) => println!("Pasted {:?}", data),
            Event::Resize(width, height) => println!("New size {}x{}", width, height),
        };
        render(&lines).unwrap();
    }
}
