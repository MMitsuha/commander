use crossterm::event::KeyEvent;
use ratatui::{
    style::{Style, Stylize},
    widgets::{Block, BorderType, Borders},
};
use std::error;
use tui_textarea::TextArea;

/// Application result type.
pub type AppResult<T> = std::result::Result<T, Box<dyn error::Error>>;

/// Application.
#[derive(Debug)]
pub struct App<'a> {
    /// Is the application running?
    pub running: bool,
    /// Text area
    pub command_line: TextArea<'a>,
    /// Output area
    pub output_area: TextArea<'a>,
}

impl Default for App<'_> {
    fn default() -> Self {
        let mut command_line = TextArea::default();
        command_line.set_cursor_line_style(Style::default());
        command_line.set_block(
            Block::default()
                .borders(Borders::ALL)
                .title("Command")
                .border_type(BorderType::Rounded),
        );

        let mut output_area = TextArea::default();
        output_area.set_max_histories(0);
        output_area.set_block(
            Block::default()
                .borders(Borders::ALL)
                .title("Terminal")
                .border_type(BorderType::Rounded),
        );

        Self {
            running: true,
            command_line,
            output_area,
        }
    }
}

impl App<'_> {
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

    pub fn input(&mut self, key: KeyEvent) {
        self.command_line.input(key);
    }

    pub fn enter(&mut self) {
        for line in self.command_line.lines() {
            self.output_area.insert_str("> ");
            self.output_area.insert_str(line);
            self.output_area.insert_newline();
        }

        self.command_line.delete_line_by_head();
    }

    pub fn scroll_up(&mut self) {
        self.output_area.scroll((-1, 0));
    }

    pub fn scroll_down(&mut self) {
        self.output_area.scroll((1, 0));
    }

    pub fn undo_command(&mut self) {
        self.command_line.undo();
    }

    pub fn redo_command(&mut self) {
        self.command_line.redo();
    }
}
