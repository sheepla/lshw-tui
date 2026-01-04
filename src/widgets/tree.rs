use ratatui::{
    layout::Alignment,
    style::{Color, Style},
    widgets::{Borders, StatefulWidget, Widget},
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

        match Tree::new(self.items) {
            Ok(tree) => {
                let tree_widget = tree
                    .block(block)
                    .highlight_style(highlight_style)
                    .highlight_symbol(">> ");
                StatefulWidget::render(tree_widget, area, buf, state);
            }
            Err(_) => {
                Widget::render(block, area, buf);
            }
        }
    }
}
