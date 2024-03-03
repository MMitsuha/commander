use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Style},
    text::{Line, Text},
    widgets::{Block, BorderType, Borders, Paragraph},
    Frame,
};

use crate::app::App;

/// Renders the user interface widgets.
pub fn render(app: &mut App, frame: &mut Frame) {
    // This is where you add new widgets.
    // See the following resources:
    // - https://docs.rs/ratatui/latest/ratatui/widgets/index.html
    // - https://github.com/ratatui-org/ratatui/tree/master/examples
    let layout = Layout::new(
        Direction::Vertical,
        [Constraint::Min(1), Constraint::Max(3)],
    )
    .split(frame.size());
    let output_layout = layout[0];
    let input_layout = layout[1];

    let output = Block::new();
    let history_layout = output.inner(output_layout);
    let history = Text::default();

    let input = Block::new()
        .borders(Borders::all())
        .border_type(BorderType::Rounded);
    let command_layout = input.inner(input_layout);
    let command = Line::raw(&app.command_line);

    frame.render_widget(output, output_layout);
    frame.render_widget(input, input_layout);
    frame.render_widget(command, command_layout);
    frame.render_widget(history, history_layout);
}
