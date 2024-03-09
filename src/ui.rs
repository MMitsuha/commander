use ratatui::{
    layout::{Constraint, Direction, Layout},
    Frame,
};

use crate::app::App;

/// Renders the user interface widgets.
pub fn render(app: &mut App, frame: &mut Frame) {
    let layout = Layout::new(
        Direction::Vertical,
        [Constraint::Min(1), Constraint::Max(3)],
    )
    .split(frame.size());
    let output_layout = layout[0];
    let input_layout = layout[1];

    let output = &mut app.output_area;

    let command = &mut app.command_line;

    frame.render_widget(output.widget(), output_layout);
    frame.render_widget(command.widget(), input_layout);
}
