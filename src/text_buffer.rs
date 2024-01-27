pub struct TextBuffer {
    pub lines: Vec<String>,
    history: Vec<Vec<ChangeType>>,
    current_state_index: usize,
    create_new_change_frame: bool,
}

struct Change {
    element: String,
    index: usize,
}

enum ChangeType {
    Insert(Change),
    Remove(Change),
}

impl TextBuffer {
    pub fn new(lines: Vec<String>) -> TextBuffer {
        return TextBuffer {
            lines,
            history: Vec::new(),
            current_state_index: 0,
            create_new_change_frame: true,
        };
    }

    pub fn insert(&mut self, index: usize, element: String) {
        let change = Change {
            element: element.clone(),
            index,
        };
        self.record(ChangeType::Insert(change));
        self.lines.insert(index, element);
    }

    pub fn remove(&mut self, index: usize) -> String {
        let change = Change {
            element: self.lines[index].clone(),
            index,
        };
        self.record(ChangeType::Remove(change));
        self.lines.remove(index)
    }

    fn record(&mut self, change_type: ChangeType) {
        self.history.truncate(self.current_state_index + 1);
        if self.create_new_change_frame || self.history.len() == 0 {
            self.history.push(Vec::new());
            self.current_state_index = self.history.len() - 1;
            self.create_new_change_frame = false;
        }
        self.history.last_mut().unwrap().push(change_type);
    }

    pub fn new_change_frame(&mut self) {
        self.create_new_change_frame = true;
    }

    pub fn undo(&mut self) {
        if self.current_state_index == 0 {
            return;
        };
        for change_type in self.history[self.current_state_index].iter().rev() {
            match change_type {
                ChangeType::Insert(change) => {
                    self.lines.remove(change.index);
                }
                ChangeType::Remove(change) => {
                    self.lines.insert(change.index, change.element.clone());
                }
            };
        }
        self.current_state_index -= 1;
    }

    pub fn redo(&mut self) {
        if self.current_state_index == self.history.len() {
            return;
        };
        for change_type in self.history[self.current_state_index].iter().rev() {
            match change_type {
                ChangeType::Insert(change) => {
                    self.lines.insert(change.index, change.element.clone());
                }
                ChangeType::Remove(change) => {
                    self.lines.remove(change.index);
                }
            };
        }
        self.current_state_index += 1;
    }
}
