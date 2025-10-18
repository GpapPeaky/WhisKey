// Command indeces.
// After reading the command string
// we will return one of these enums
// that will be used as indeces to 
// an array of functions
//
// The special case where we use the 
// 'fname' -w
//  or
// 'fname'
// we will return the enumerator value and handle
// it accordingly elsewhere
pub enum COMMAND_INDECES {
    CommandUnkown = -4, // Invalid command

    CommandSwitchAndSaveCurrentFile = -3,
    CommandSwitchCurrentFile = -2,
    CommandFileHandle = -1,

    CommandSwitchDirectory = 0,
    CommandWriteCurrentFile,
    CommandDeleteFile,
    CommandExit,
    CommandPalleteSwitch,
    CommandGoToLine 
}

// Command name vector
// we will check here when for a '?' character
// character is found in an inputed line
// (when in console mode)
pub const COMMAND_VECTOR: [&str ; 6] = [
    "cd",
    "wf",
    "rf",
    "e",
    "p",
    "l"
];

// Command struct
pub struct ConsoleCommand {
    pub command_index: i8,      // Index assigned
    pub text: String            // Command string, when split we will find the command and its parameters
}

impl ConsoleCommand {

    // Command constructor
    pub fn new() -> Self {
        Self { 
            command_index: 0,
            text: String::new(),
        }
    }

    // Exit the process, kills the editor
    pub fn exit() {
        std::process::exit(COMMAND_INDECES::CommandExit as i32);
    }

}
