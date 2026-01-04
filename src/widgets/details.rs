use ratatui::{
    prelude::*,
    text::Line,
    widgets::{Borders, Padding, Paragraph, Widget},
};
use serde_json::Value;

use crate::core::types::HardwareNode;

pub struct DetailsWidget<'a> {
    node: Option<&'a HardwareNode>,
}

impl<'a> DetailsWidget<'a> {
    pub fn new(node: Option<&'a HardwareNode>) -> Self {
        Self { node }
    }
}

impl<'a> Widget for DetailsWidget<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let block = ratatui::widgets::Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::DarkGray))
            .title_alignment(Alignment::Center)
            .title_style(Style::new().fg(Color::White))
            .title(" Details ");

        let paragraph = if let Some(node) = self.node {
            let mut lines = Vec::new();
            add_line("ID", &Some(Value::String(node.id.clone())), &mut lines);
            add_line(
                "Class",
                &Some(Value::String(node.class.clone())),
                &mut lines,
            );
            add_line("Description", &node.description, &mut lines);
            add_line("Product", &node.product, &mut lines);
            add_line("Vendor", &node.vendor, &mut lines);
            add_line("Physid", &node.physid, &mut lines);
            add_line("Logical Name", &node.logicalname, &mut lines);
            add_line("Dev", &node.dev, &mut lines);
            add_line("Bus Info", &node.businfo, &mut lines);
            add_line("Version", &node.version, &mut lines);
            add_line("Date", &node.date, &mut lines);
            add_line("Serial", &node.serial, &mut lines);
            add_line("Slot", &node.slot, &mut lines);
            add_line("Handle", &node.handle, &mut lines);
            add_line("Size", &node.size, &mut lines);
            add_line("Capacity", &node.capacity, &mut lines);
            add_line("Units", &node.units, &mut lines);
            add_line("Width", &node.width, &mut lines);
            add_line("Clock", &node.clock, &mut lines);
            add_line("Claimed", &node.claimed, &mut lines);
            add_line("Disabled", &node.disabled, &mut lines);
            add_line("Capabilities", &node.capabilities, &mut lines);
            add_line("Configuration", &node.configuration, &mut lines);

            Paragraph::new(lines).block(block)
        } else {
            Paragraph::new("No item selected").block(block)
        };

        paragraph.render(area, buf);
    }
}

fn add_line(label: &str, value: &Option<Value>, lines: &mut Vec<Line>) {
    if let Some(val) = value {
        let val_str = val.to_string();
        if !val_str.is_empty() {
            lines.push(Line::from(vec![
                Span::styled(format!("{}: ", label), Style::default().bold()),
                Span::raw(val_str),
            ]));
        }
    }
}
