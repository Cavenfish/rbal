use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Rect},
    style::{Modifier, Style, Stylize},
    text::Text,
    widgets::{Cell, Row, StatefulWidget, Table, TableState},
};

use super::theme::TableTheme;
use crate::{args::TransInfo, utils::get_rows};

pub struct TableTab {
    colors: TableTheme,
    items: Vec<TransInfo>,
}

impl Default for TableTab {
    fn default() -> Self {
        let items = get_rows();

        Self {
            colors: TableTheme::default(),
            items,
        }
    }
}

impl StatefulWidget for TableTab {
    type State = TableState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        let header_style = Style::default()
            .fg(self.colors.header_fg)
            .bg(self.colors.header_bg);

        let header = ["Vendor", "Date", "Coin", "Amount"]
            .into_iter()
            .map(Cell::from)
            .collect::<Row>()
            .style(header_style)
            .height(1);

        let rows = self.items.into_iter().enumerate().map(|(i, data)| {
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
        let highlight_style = Style::default().add_modifier(Modifier::REVERSED);

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
        .row_highlight_style(highlight_style)
        .bg(self.colors.background)
        .render(area, buf, state);
    }
}
