use macroquad::prelude::*;

#[path = "command.rs"]
mod command;
use command::ConsoleCommand;
use crate::console::command::{COMMAND_VECTOR, COMMAND_INDECES};

// Console struct.
// Handles general directives like:
// 'CTRL + `': Opens/closes the console
//
// 'fname' -w: Typing the name of a file, writes the current file and switches over to it
// if found, else it asks to create it
//
// 'fname': Typing the name of a file, switches over to it without saving
// if found, else it asks to create it
//
// '?cd': Switch cwd -> call upon windows to open the folder panel
//
// '?wf': Save the currently open file
//
// '?rf fname': Delete a file with name 'fname'
//
// '?e': Exit the editor, terminate the program
//
// '?p pname': Pallete switch to a pallete with name 'pname'
//
// '?l lnum': Go to line lnum in the current file
pub struct Console {
    pub console_mode: bool,             // Switch in and out of the console
    pub command: command::ConsoleCommand,        // Command object
    pub cursor_x: usize                 // Cursor position inside the command
}

// Console height
pub const CONSOLE_HEIGHT: f32 = 150.0;

// Console font size
const console_font_size: f32 = 30.0;

impl Console {

    // Console constructor
    pub fn new() -> Self {
       Self{
            console_mode: false,
            command: ConsoleCommand::new(),
            cursor_x: 0
       }    
    }

    // Switch in and out of the console
    pub fn console_mode_switch(&mut self) {
        self.console_mode = !self.console_mode;
    }

    // Take input in the console and
    // insert a character via keypress
    pub fn insert_char(&mut self, c: char) {
        if self.cursor_x <= self.command.text.len() {
            self.command.text.insert(self.cursor_x, c);
            self.cursor_x += 1;
        }
    }

    // Backspace
    pub fn backspace(&mut self) {
        if self.cursor_x > 0 {
            self.cursor_x -= 1;
            self.command.text.remove(self.cursor_x);
        }
    }

    // Execute command, via the return/enter key press
    pub fn execute(&mut self) {
        let index = self.read_command();

        if index == COMMAND_INDECES::CommandExit as i32 {
            self.command.exit();
        }

    }

    pub fn read_command(&self) -> i32 {
        // Handled elsewhere
        if !self.command.text.starts_with('?') {
            return COMMAND_INDECES::CommandFileHandle as i32;
        }

        // Trim '?' to match to to whatever
        let cmd = self.command.text.trim_start_matches('?');

        // Normal '?' command fallback
        for (i, command) in COMMAND_VECTOR.iter().enumerate() {
            if cmd.starts_with(command) {
                return i as i32; // return the command index
            }
        }

        COMMAND_INDECES::CommandUnkown as i32
    }

    // Render the console promt
    pub fn render_console(&mut self) {

        // Always at the bottom of the screen
        if self.console_mode {
            // Display that we are inside the console
            draw_text("CONSOLE MODE",
                screen_width() - 132.5,
                20.0,
                25.0,
                WHITE
            );

            // Display the bottom bar
            // Move cursor there
            // Take text there

            // Console rectangle
            draw_rectangle(0.0, screen_height() - CONSOLE_HEIGHT, screen_width(), CONSOLE_HEIGHT,BLACK);

            // Seperator from the file
            draw_line(0.0,
                screen_height() - CONSOLE_HEIGHT,
                screen_width(),
                screen_height() - CONSOLE_HEIGHT,
                2.5, WHITE);

                // From main.rs
                /*
                    // Draw the actual text
                    draw_text_ex(
                        line.as_str(),
                        65.0,                       // Shift text to the right to leave space for numbers
                        top_bar_margin + 20.0 + i as f32 * font_size as f32,
                        TextParams {
                            font: Some(&font),
                            font_size,
                            color: WHITE,
                            ..Default::default()
                        },
                    );
                */

            draw_text(self.command.text.as_str(),
                5.0, 
                screen_height() - 
                    CONSOLE_HEIGHT + 
                    console_font_size 
                , console_font_size, 
                WHITE);
        } else {
            draw_text("TEXT MODE",
                screen_width() - 100.0,
                20.0,
                25.0,
                WHITE
            );
        }
    }

}
