use crate::{coordinate::Coordinate, view::View};

#[derive(Clone)]
pub struct Cursor {
    pub x: usize,
    pub y: usize,
    // Virtual x keeps track of x if you only move y. This is used to avoid
    // changing x position if moving to and from a shorter line.
    virtual_x: usize,
}

impl Cursor {
    pub fn new(x: usize, y: usize) -> Self {
        Cursor { x, y, virtual_x: x }
    }

    pub fn move_x(&mut self, delta: isize, lines: &Vec<String>) {
        self.x = self.virtual_x.min(lines[self.y].len());
        self.x = match self.x.checked_add_signed(delta) {
            Some(new_x) => new_x,
            None => 0,
        };
        self.x = self.x.min(lines[self.y].len());
        self.virtual_x = self.x;
    }

    pub fn move_y(&mut self, delta: isize, lines: &Vec<String>) {
        self.y = match self.y.checked_add_signed(delta) {
            Some(new_y) => new_y,
            None => 0,
        };
        self.y = self.y.min(lines.len() - 1);
        self.x = self.virtual_x.min(lines[self.y].len());
    }

    pub fn set_x(&mut self, new_x: usize, lines: &Vec<String>) {
        self.x = new_x;
        self.x = self.x.min(lines[self.y].len());
        self.virtual_x = self.x;
    }

    pub fn set_y(&mut self, new_y: usize, lines: &Vec<String>) {
        self.y = new_y;
        self.y = self.y.min(lines.len() - 1);
        self.x = self.virtual_x.min(lines[self.y].len());
    }

    pub fn get_render_position(&self, view: &View) -> Option<Coordinate<usize>> {
        let x;
        let y;
        if view.h_scroll <= self.x && self.x < (view.h_scroll + view.width) {
            x = self.x - view.h_scroll;
        } else {
            return None;
        };

        if view.v_scroll <= self.y && self.y < (view.v_scroll + view.height) {
            y = self.y - view.v_scroll;
        } else {
            return None;
        };
        return Some(Coordinate { x, y });
    }
}
