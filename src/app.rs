use std::error;

/// Application result type.
pub type AppResult<T> = std::result::Result<T, Box<dyn error::Error>>;

/// Application.
#[derive(Debug)]
pub struct App {
    /// Is the application running?
    pub running: bool,
    /// bottom command line
    pub command_line: String,
    /// cursor position
    pub cursor: usize,
}

impl Default for App {
    fn default() -> Self {
        Self {
            running: true,
            command_line: String::new(),
            cursor: 0,
        }
    }
}

impl App {
    /// Constructs a new instance of [`App`].
    pub fn new() -> Self {
        Self::default()
    }

    /// Handles the tick event of the terminal.
    pub fn tick(&self) {}

    /// Set running to false to quit the application.
    pub fn quit(&mut self) {
        self.running = false;
    }

    pub fn press_char(&mut self, key: char) {
        self.command_line.insert_str(self.cursor, &key.to_string());
        self.rightwards_cursor();
    }

    pub fn backspace_char(&mut self) {
        self.command_line.remove(self.cursor - 1);
        self.leftwards_cursor();
    }

    pub fn delete_char(&mut self) {
        if self.cursor < self.command_line.len() {
            self.command_line.remove(self.cursor);
        }
    }

    pub fn rightwards_cursor(&mut self) {
        if let Some(cursor) = self.cursor.checked_add(1) {
            self.cursor = cursor;
        }
    }

    pub fn leftwards_cursor(&mut self) {
        if let Some(cursor) = self.cursor.checked_sub(1) {
            self.cursor = cursor;
        }
    }
}
