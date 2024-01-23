use std::{
    env,
    fs::File,
    io::{self, stdout, BufRead, BufReader, Write},
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

impl Cursor {
    fn move_cursor_x(&mut self, delta: isize, lines: &Vec<String>) {
        self.x = match self.x.checked_add_signed(delta) {
            Some(new_x) => new_x,
            None => 0,
        };
        self.x = self.x.min(lines[self.y].len())
    }

    fn move_cursor_y(&mut self, delta: isize, lines: &Vec<String>) {
        self.y = match self.y.checked_add_signed(delta) {
            Some(new_y) => new_y,
            None => 0,
        };
        self.y = self.y.min(lines.len());
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    let mut lines = read_lines(&file_path).unwrap();

    enable_raw_mode().expect("Failed to enable raw mode");
    event_loop(&mut lines).unwrap();
    disable_raw_mode().expect("Failed to disable raw mode");
}

fn render(lines: &Vec<String>, cursor: &Cursor) -> io::Result<()> {
    let mut stdout = stdout();
    stdout.queue(cursor::MoveTo(0, 0))?;
    stdout.queue(terminal::Clear(terminal::ClearType::All))?;

    for line in lines {
        stdout.queue(style::Print(format!("{}\r\n", line)))?;
    }
    stdout.queue(cursor::MoveTo(cursor.x as u16, cursor.y as u16))?;
    stdout.queue(cursor::Show)?;

    stdout.flush()?;
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
                KeyCode::Up => cursor.move_cursor_y(-1, &lines),
                KeyCode::Down => cursor.move_cursor_y(1, &lines),
                KeyCode::Left => cursor.move_cursor_x(-1, &lines),
                KeyCode::Right => cursor.move_cursor_x(1, &lines),
                KeyCode::Esc => break,
                _ => (),
            },
            Event::Mouse(event) => println!("{:?}", event),
            Event::Paste(data) => println!("Pasted {:?}", data),
            Event::Resize(width, height) => println!("New size {}x{}", width, height),
        };
        render(&lines, &cursor).unwrap();
    }
    Ok(())
}
