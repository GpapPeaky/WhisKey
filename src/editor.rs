use macroquad::{input::KeyCode, text};

// Basic editor struct
pub struct Editor{
    pub scope: i32,        // Current scope
    pub text: Vec<String>, // File text string
    pub cursor_x: usize,   // Current cursor position x
    pub cursor_y: usize    // Current cursor position y
}

const TAB_SIZE: usize = 4;

impl Editor {
    
    // Constructor
    pub fn new() -> Self {
        Self {
            scope: 0,
            text: vec![String::new()],
            cursor_x: 0,
            cursor_y: 0
        }
    }

    // Insert a character via keypress
    pub fn insert_char(&mut self, c: char) {
        if let Some(line) = self.text.get_mut(self.cursor_y) {
            let special_char = Self::special_char_insertion(c, line, &mut self.cursor_x);
            
            if !special_char {
                line.insert(self.cursor_x, c);
                self.cursor_x += 1;
            }
        }
    }

    // Delete the previous character
   pub fn backspace(&mut self) {
    if self.cursor_x > 0 {
        let mut tab_check = false;

        // Short immutable borrow
        if self.cursor_x >= TAB_SIZE {
            if let Some(line) = self.text.get(self.cursor_y) {
                let start = self.cursor_x - TAB_SIZE;
                let end = self.cursor_x;
                tab_check = line.get(start..end) == Some("    "); // 4 spaces
            }
        }

        // Mutable borrow starts after immutable borrow ends
        if let Some(line) = self.text.get_mut(self.cursor_y) {
            if tab_check {
                let start = self.cursor_x - TAB_SIZE;
                let end = self.cursor_x;
                if start < end && end <= line.len() {
                    line.drain(start..end); // safely remove that slice
                    self.cursor_x -= TAB_SIZE;
                }
            } else if self.cursor_x > 0 && self.cursor_x <= line.len() {
                line.remove(self.cursor_x - 1);
                self.cursor_x -= 1;
            }
        }
    } else if self.cursor_y > 0 {
        // merge with previous line
        let removed_line = self.text.remove(self.cursor_y);
        self.cursor_y -= 1;
        if let Some(prev_line) = self.text.get_mut(self.cursor_y) {
            self.cursor_x = prev_line.len();
            prev_line.push_str(&removed_line);
        }
    }
}

    // Enter a new line
    pub fn new_line(&mut self) {
        // Get mutable reference to the current line
        let current_line = &mut self.text[self.cursor_y];
    
        // Split the current line at the cursor
        let rest = current_line.split_off(self.cursor_x);
    
        // Move cursor to start of new line
        self.cursor_y += 1;
        self.cursor_x = 0;
    
        // Insert a new line with the rest of the text
        self.text.insert(self.cursor_y, rest);
    }

    // Check for special character insertions
    // like when pressing '(' it creates another ')'
    // or the tab -> <3 spaces>
    pub fn special_char_insertion(c: char, line: &mut String, cursor_x: &mut usize) -> bool {
        if c == '(' {
            line.insert(*cursor_x, c);
            line.insert(*(cursor_x) + 1, ')');

            *cursor_x += 1;

            return true;
        }

        if c == '[' {
            line.insert(*cursor_x, c);
            line.insert(*(cursor_x) + 1, ']');

            *cursor_x += 1;

            return true;
        }

        if c == '{' {
            line.insert(*cursor_x, c);
            line.insert(*(cursor_x) + 1, '}');

            *cursor_x += 1;

            return true;
        }

        if c == '"' {
            line.insert(*cursor_x, c);
            line.insert(*(cursor_x) + 1, '"');

            *cursor_x += 1;

            return true;
        }

        if c == '\'' {
            line.insert(*cursor_x, c);
            line.insert(*(cursor_x) + 1, '\'');

            *cursor_x += 1;

            return true;
        }

        false
    }

    // Tab insertion

    pub fn insert_tab(&mut self) {
        if let Some(line) = self.text.get_mut(self.cursor_y) {
            for i in 0..TAB_SIZE {
                line.insert(self.cursor_x + i, ' ');
            }

            self.cursor_x += TAB_SIZE;
        }
    }

    // Move the cursor
    pub fn move_cursor(&mut self, key: KeyCode) {
        match key {
            KeyCode::Up => {
                if self.cursor_y > 0 {
                    self.cursor_y -= 1;
                    self.cursor_x = self.cursor_x.min(self.text[self.cursor_y].len());
                }
            }
            KeyCode::Down => {
                if self.cursor_y + 1 < self.text.len() {
                    self.cursor_y += 1;
                    self.cursor_x = self.cursor_x.min(self.text[self.cursor_y].len());
                }
            }
            KeyCode::Left => {
                if self.cursor_x > 0 {
                    self.cursor_x -= 1;
                } else if self.cursor_y > 0 {
                    self.cursor_y -= 1;
                    self.cursor_x = self.text[self.cursor_y].len();
                }
            }
            KeyCode::Right => {
                if self.cursor_x < self.text[self.cursor_y].len() {
                    self.cursor_x += 1;
                } else if self.cursor_y + 1 < self.text.len() {
                    self.cursor_y += 1;
                    self.cursor_x = 0;
                }
            }
            _ => {}
        }
    }
}