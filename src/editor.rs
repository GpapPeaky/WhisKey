use macroquad::{input::KeyCode};

// Basic editor struct.
// Handles cursor positions
// and text editing
pub struct Editor {
    pub text: Vec<String>, // File text string
    pub cursor_x: usize,   // Current cursor position x
    pub cursor_y: usize    // Current cursor position y
}

const TAB_SIZE: usize = 4;

impl Editor {
    
    // Editor constructor
    pub fn new() -> Self {
        Self {
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
        if let Some(current_line) = self.text.get_mut(self.cursor_y) {
            // Split the current line at the cursor position
            let rest = current_line.split_off(self.cursor_x);
    
            // Count leading spaces on current line (indentation)
            let leading_spaces = current_line.chars().take_while(|c| *c == ' ').count();
    
            // Check characters around the cursor
            let before = if self.cursor_x > 0 {
                current_line.chars().nth(self.cursor_x - 1)
            } else {
                None
            };
            let after = rest.chars().next();
    
            // Are we between { and } ?
            let between_braces = before == Some('{') && after == Some('}');
    
            // Move cursor to next line
            self.cursor_y += 1;
    
            if between_braces {
                // One level deeper than the previous indentation
                let inner_indent = " ".repeat(leading_spaces + TAB_SIZE);
                let outer_indent = " ".repeat(leading_spaces);
    
                // Insert the new indented empty line
                self.text.insert(self.cursor_y, inner_indent.clone());
                // Insert a line below it with the closing brace and preserved outer indent
                self.text.insert(self.cursor_y + 1, format!("{}{}", outer_indent, rest));
    
                // Place cursor at the new indented position
                self.cursor_x = leading_spaces + TAB_SIZE;
            } else {
                // Check if line ends with '{' → increase indent
                let mut new_indent = leading_spaces;
                if before == Some('{') {
                    new_indent += TAB_SIZE;
                }
    
                let indent = " ".repeat(new_indent);
                self.text.insert(self.cursor_y, format!("{}{}", indent, rest));
                self.cursor_x = new_indent;
            }
        }
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

    // Control character handling
    pub fn control_key_handle(&mut self, key: KeyCode){
        
    }
}