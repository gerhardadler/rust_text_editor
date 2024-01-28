use crate::cursor::Cursor;

pub struct TextBuffer {
    pub lines: Vec<String>,
    history: Vec<ChangeFrame>,
    current_state_index: usize,
    new_change_frame: Option<ChangeFrame>,
}

struct Change {
    element: String,
    index: usize,
}

enum ChangeType {
    Insert(Change),
    Remove(Change),
}

struct ChangeFrame {
    change_types: Vec<ChangeType>,
    cursor: Cursor,
}

impl TextBuffer {
    pub fn new(lines: Vec<String>) -> TextBuffer {
        return TextBuffer {
            lines,
            history: Vec::new(),
            current_state_index: 0,
            new_change_frame: Some(ChangeFrame {
                change_types: Vec::new(),
                cursor: Cursor::new(0, 0),
            }),
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

        let new_change_frame = std::mem::replace(&mut self.new_change_frame, None);
        if let Some(change_frame) = new_change_frame {
            self.history.push(change_frame);
            self.current_state_index = self.history.len() - 1;
        }
        self.history
            .last_mut()
            .unwrap()
            .change_types
            .push(change_type);
    }

    pub fn new_change_frame(&mut self, cursor: Cursor) {
        self.new_change_frame = Some(ChangeFrame {
            change_types: Vec::new(),
            cursor,
        });
    }

    pub fn undo(&mut self, cursor: &mut Cursor) {
        if self.current_state_index == 0 {
            return;
        };
        let change_frame = &self.history[self.current_state_index];
        for change_type in change_frame.change_types.iter().rev() {
            match change_type {
                ChangeType::Insert(change) => {
                    self.lines.remove(change.index);
                }
                ChangeType::Remove(change) => {
                    self.lines.insert(change.index, change.element.clone());
                }
            };
        }
        let _ = std::mem::replace(cursor, change_frame.cursor.clone());
        self.current_state_index -= 1;
    }

    pub fn redo(&mut self, cursor: &mut Cursor) {
        if self.current_state_index == self.history.len() {
            return;
        };
        let change_frame = &self.history[self.current_state_index];
        for change_type in change_frame.change_types.iter().rev() {
            match change_type {
                ChangeType::Insert(change) => {
                    self.lines.insert(change.index, change.element.clone());
                }
                ChangeType::Remove(change) => {
                    self.lines.remove(change.index);
                }
            };
        }
        let _ = std::mem::replace(cursor, change_frame.cursor.clone());
        self.current_state_index += 1;
    }
}
