use ratatui::prelude::*;

use crate::app::App;
use crate::widgets::details::DetailsWidget;
use crate::widgets::tree::TreeWidget;

pub fn render_screen(app: &mut App, frame: &mut Frame) {
    let layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Fill(3), Constraint::Fill(5)])
        .split(frame.area());

    // Tree Widget
    let tree_widget = TreeWidget::new(&app.tree_items);
    frame.render_stateful_widget(tree_widget, layout[0], &mut app.state.tree_state);

    // Details Widget
    let selected_node = app.get_selected_node();
    let details_widget = DetailsWidget::new(selected_node);

    frame.render_widget(details_widget, layout[1]);
}
