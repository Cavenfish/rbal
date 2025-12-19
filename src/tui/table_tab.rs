use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Rect},
    style::{Style, Stylize},
    text::Text,
    widgets::{Cell, Row, Table, Widget},
};

use super::theme::TableTheme;
use crate::utils::get_rows;

pub struct TableTab {
    colors: TableTheme,
}

impl Default for TableTab {
    fn default() -> Self {
        Self {
            colors: TableTheme::default(),
        }
    }
}

impl Widget for TableTab {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let header_style = Style::default()
            .fg(self.colors.header_fg)
            .bg(self.colors.header_bg);

        let header = ["Vendor", "Date", "Coin", "Amount"]
            .into_iter()
            .map(Cell::from)
            .collect::<Row>()
            .style(header_style)
            .height(1);

        let items = get_rows();

        let rows = items.into_iter().enumerate().map(|(i, data)| {
            let color = match i % 2 {
                0 => self.colors.background,
                _ => self.colors.alt_bg,
            };
            let item = [data.vendor, data.date, data.coin, data.amount.to_string()];

            item.into_iter()
                .map(|content| Cell::from(Text::from(content)))
                .collect::<Row>()
                .style(Style::new().fg(self.colors.foreground).bg(color))
                .height(1)
        });

        Table::new(
            rows,
            [
                Constraint::Length(16),
                Constraint::Min(16),
                Constraint::Min(11),
                Constraint::Min(6),
            ],
        )
        .header(header)
        .bg(self.colors.background)
        .render(area, buf);
    }
}
