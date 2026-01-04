use ratatui::{
    layout::Alignment,
    style::{Color, Modifier, Style},
    widgets::{Borders, Padding, StatefulWidget},
};
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
        let block = ratatui::widgets::Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::DarkGray))
            .title_alignment(Alignment::Center)
            .title_style(Style::new().fg(Color::White))
            .title(" Hardware Tree ");
        let highlight_style = Style::default()
            .fg(ratatui::style::Color::Black)
            .bg(ratatui::style::Color::LightCyan);
        let tree_widget = Tree::new(self.items)
            .unwrap()
            .block(block)
            .highlight_style(highlight_style)
            .highlight_symbol(">> ");

        tree_widget.render(area, buf, state);
    }
}
