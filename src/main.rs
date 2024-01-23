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
    // Virtual x keeps track of x if you only move y. This is used to avoid
    // changing x position if moving to and from a shorter line.
    virtual_x: usize,
}

impl Cursor {
    fn new(x: usize, y: usize) -> Self {
        Cursor { x, y, virtual_x: x }
    }

    fn move_x(&mut self, delta: isize, lines: &Vec<String>) {
        self.x = match self.virtual_x.checked_add_signed(delta) {
            Some(new_x) => new_x,
            None => 0,
        };
        self.x = self.x.min(lines[self.y].len() - 1);
        self.virtual_x = self.x;
    }

    fn move_y(&mut self, delta: isize, lines: &Vec<String>) {
        self.y = match self.y.checked_add_signed(delta) {
            Some(new_y) => new_y,
            None => 0,
        };
        self.y = self.y.min(lines.len() - 1);
        self.x = self.virtual_x.min(lines[self.y].len() - 1)
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
    let mut cursor = Cursor::new(0, 0);
    loop {
        match read()? {
            Event::FocusGained => println!("FocusGained"),
            Event::FocusLost => println!("FocusLost"),
            Event::Key(event) => match event.code {
                KeyCode::Char(char) => match lines.get_mut(cursor.y) {
                    Some(line) => {
                        line.insert(cursor.x, char);
                        cursor.move_x(1, &lines);
                    }
                    None => (),
                },
                KeyCode::Up => cursor.move_y(-1, &lines),
                KeyCode::Down => cursor.move_y(1, &lines),
                KeyCode::Left => cursor.move_x(-1, &lines),
                KeyCode::Right => cursor.move_x(1, &lines),
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
