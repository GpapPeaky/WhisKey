use macroquad::prelude::*;

// Console struct.
// Handles general commands like:
// 'CTRL + `': Opens/closes the console
//
// 'fname': Typing the name of a file, switches over to it
// if found, else it asks to create it
//
// '~sd': Switch cwd -> call upon windows to open the folder panel
//
// '~sf': Save the currently open file
//
// '~df fname': Delete a file with name 'fname'
//
// '~e': Exit the editor, terminate the program
//
// '~p pname': Pallete switch to a pallete with name 'pname'
pub struct Console {
    pub console_mode: bool, // Switch in and out of the console
    pub command: String     // Commands string
}

// Command name vector
// we will check here when a '`'
// character is found in an inputed line
// (when in console mode)
const command_vector: [&str ; 5] = [
    "sd",
    "sf",
    "df",
    "e",
    "p"
];

impl Console {

    // Console constructor
    pub fn new() -> Self {
       Self{
            console_mode: false,
            command: String::new()
       }    
    }

    // Switch in and out of the console
    pub fn console_mode_switch(&mut self) {
        self.console_mode = !self.console_mode;
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