pub struct TextBuffer {
    pub lines: Vec<String>,
    history: Vec<ChangeType>,
    current_state_index: usize,
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
    pub fn insert(&mut self, index: usize, element: String) {
        let change = Change {
            element: element.clone(),
            index,
        };
        self.record(ChangeType::Insert(change));
        self.lines.insert(index, element);
    }

    pub fn remove(&mut self, index: usize) {
        let change = Change {
            element: self.lines[index].clone(),
            index,
        };
        self.record(ChangeType::Remove(change));
        self.lines.remove(index);
    }

    fn record(&mut self, change_type: ChangeType) {
        self.history.truncate(self.current_state_index + 1);
        self.history.push(change_type);
        self.current_state_index = self.history.len() - 1;
    }

    pub fn undo(&mut self) {
        if self.current_state_index > 0 {
            self.current_state_index -= 1;
            match &self.history[self.current_state_index] {
                ChangeType::Insert(change) => {
                    self.lines.remove(change.index);
                }
                ChangeType::Remove(change) => {
                    self.lines.insert(change.index, change.element.clone());
                }
            };
        }
    }

    pub fn redo(&mut self) {
        if self.current_state_index < self.history.len() - 1 {
            self.current_state_index += 1;
            match &self.history[self.current_state_index] {
                ChangeType::Insert(change) => {
                    self.lines.insert(change.index, change.element.clone());
                }
                ChangeType::Remove(change) => {
                    self.lines.remove(change.index);
                }
            };
        }
    }
}
