use crate::text_buffer::{self, TextBuffer};

pub struct View {
    pub width: usize,
    pub height: usize,
    pub v_scroll: usize,
    pub h_scroll: usize,
}

impl View {
    pub fn move_v_scroll(&mut self, delta: isize, lines: &Vec<String>) {
        self.v_scroll = match self.v_scroll.checked_add_signed(delta) {
            Some(new_v_scroll) => new_v_scroll,
            None => 0,
        };
        self.v_scroll = self.v_scroll.min(lines.len() - 1);
    }

    pub fn move_h_scroll(&mut self, delta: isize, lines: &Vec<String>) {
        self.h_scroll = match self.h_scroll.checked_add_signed(delta) {
            Some(new_h_scroll) => new_h_scroll,
            None => 0,
        };
    }

    pub fn set_v_scroll(&mut self, new: usize, lines: &Vec<String>) {
        self.v_scroll = new;
        self.v_scroll = self.v_scroll.min(lines.len() - 1);
    }

    pub fn set_h_scroll(&mut self, new: usize, lines: &Vec<String>) {
        self.h_scroll = new;
    }
}
