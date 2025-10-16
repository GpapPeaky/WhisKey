use macroquad::input::KeyCode;

// Basic editor struct
pub struct Editor{
    pub text: Vec<String>, // File text string
    pub cursor_x: usize,   // Current cursor position x
    pub cursor_y: usize    // Current cursor position y
}

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
            line.insert(self.cursor_x, c);

            self.cursor_x += 1;
        }
    }

    // Delete the previous character
    pub fn backspace(&mut self) {
        if self.cursor_x > 0 {
            if let Some(line) = self.text.get_mut(self.cursor_y) {
                line.remove(self.cursor_x - 1);
                self.cursor_x -= 1;
            }
        }else if self.cursor_y > 0 {
            let removed_line = self.text.remove(self.cursor_y);
            self.cursor_y -= 1;
            self.cursor_x = self.text[self.cursor_y].len();
            self.text[self.cursor_y].push_str(&removed_line);
        }
    }

    // Enter a new line
    pub fn new_line(&mut self) {
        let rest = self.text[self.cursor_y].split_off(self.cursor_x);
        self.cursor_y += 1;
        self.cursor_x = 0;
        self.text.insert(self.cursor_y, rest);
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