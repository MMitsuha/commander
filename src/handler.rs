use crate::app::{App, AppResult};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers, MouseEvent, MouseEventKind};

/// Handles the key events and updates the state of [`App`].
pub fn handle_key_events(key_event: KeyEvent, app: &mut App) -> AppResult<()> {
    match key_event.code {
        // Exit application on `ESC` or `q`
        KeyCode::Esc => app.quit(),

        KeyCode::Enter => app.enter(),

        KeyCode::Up => {
            if key_event.modifiers == KeyModifiers::CONTROL {
                app.scroll_up()
            } else {
                app.undo_command()
            }
        }

        KeyCode::Down => {
            if key_event.modifiers == KeyModifiers::CONTROL {
                app.scroll_down()
            } else {
                app.redo_command()
            }
        }

        _ => app.input(key_event),
    }
    Ok(())
}

pub fn handle_mouse_events(mouse_event: MouseEvent, app: &mut App) -> AppResult<()> {
    match mouse_event.kind {
        MouseEventKind::ScrollUp => app.scroll_up(),

        MouseEventKind::ScrollDown => app.scroll_down(),

        _ => {}
    }

    Ok(())
}
