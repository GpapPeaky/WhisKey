use macroquad::input::KeyCode;

// Basic editor struct
pub struct Editor{
    pub text: Vec<String>, // File text string
    pub cursor_x: usize,   // Current cursor position x
    pub cursor_y: usize    // Current cursor position y
}

const TAB_SIZE: usize = 4;

impl Editor {
    
    // Constructor
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
            if let Some(line) = self.text.get_mut(self.cursor_y) {
                line.remove(self.cursor_x - 1);
                self.cursor_x -= 1;
            }
        } else if self.cursor_y > 0 {
            let removed_line = self.text.remove(self.cursor_y);
            self.cursor_y -= 1;
            self.cursor_x = self.text[self.cursor_y].len();
            self.text[self.cursor_y].push_str(&removed_line);
        }
    }

    // Enter a new line
    pub fn new_line(&mut self) {
        // compute indentation from current line
        let indentation: String = {
            let current_line = &self.text[self.cursor_y]; // immutable borrow
            current_line.chars()
                .take_while(|c| c.is_whitespace())
                .collect()
        }; // immutable borrow ends here
    
        // get mutable reference to current line
        let current_line = &mut self.text[self.cursor_y];
    
        // split off rest of the line at cursor_x
        let rest = current_line.split_off(self.cursor_x);
    
        // update cursor
        self.cursor_y += 1;
        self.cursor_x = indentation.len();
    
        // insert new line with indentation + rest
        self.text.insert(self.cursor_y, indentation + &rest);
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