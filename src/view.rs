pub struct View {
    pub width: usize,
    pub height: usize,
    pub v_scroll: usize,
    pub h_scroll: usize,
}

impl View {
    pub fn move_v_scroll(&mut self, delta: isize) {
        self.v_scroll = match self.v_scroll.checked_add_signed(delta) {
            Some(new_v_scroll) => new_v_scroll,
            None => 0,
        };
    }

    pub fn move_h_scroll(&mut self, delta: isize) {
        self.h_scroll = match self.h_scroll.checked_add_signed(delta) {
            Some(new_h_scroll) => new_h_scroll,
            None => 0,
        };
    }

    pub fn set_v_scroll(&mut self, new: usize) {
        self.v_scroll = new;
    }

    pub fn set_h_scroll(&mut self, new: usize) {
        self.h_scroll = new;
    }
}
