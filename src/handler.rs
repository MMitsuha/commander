use crate::app::{App, AppResult};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

/// Handles the key events and updates the state of [`App`].
pub fn handle_key_events(key_event: KeyEvent, app: &mut App) -> AppResult<()> {
    match key_event.code {
        // Exit application on `ESC` or `q`
        KeyCode::Esc => {
            app.quit();
        }

        KeyCode::Backspace => {
            app.backspace_char();
        }

        KeyCode::Delete => {
            app.delete_char();
        }

        KeyCode::Char(key) => {
            app.press_char(key);
        }

        KeyCode::Left => {
            app.leftwards_cursor();
        }

        KeyCode::Right => {
            app.rightwards_cursor();
        }

        _ => {}
    }
    Ok(())
}
