use ratatui::widgets::StatefulWidget;
use tui_tree_widget::{Tree, TreeItem, TreeState};

use crate::state::NodeId;

pub struct TreeWidget<'a> {
    items: &'a [TreeItem<'static, NodeId>],
}

impl<'a> TreeWidget<'a> {
    pub fn new(items: &'a [TreeItem<'static, NodeId>]) -> Self {
        Self { items }
    }
}

impl<'a> StatefulWidget for TreeWidget<'a> {
    type State = TreeState<NodeId>;

    fn render(
        self,
        area: ratatui::prelude::Rect,
        buf: &mut ratatui::prelude::Buffer,
        state: &mut Self::State,
    ) {
        let tree_widget = Tree::new(self.items)
            .unwrap()
            .block(
                ratatui::widgets::Block::default()
                    .borders(ratatui::widgets::Borders::ALL)
                    .title("Hardware Tree"),
            )
            .highlight_style(
                ratatui::style::Style::default()
                    .fg(ratatui::style::Color::Black)
                    .bg(ratatui::style::Color::LightCyan),
            )
            .highlight_symbol(">> ");

        tree_widget.render(area, buf, state);
    }
}
