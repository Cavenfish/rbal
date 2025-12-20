use std::collections::BTreeMap;

use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::Color,
    text::Line,
    widgets::{Bar, BarChart, BarGroup, Block, Widget},
};

use crate::utils::get_coins_data;

pub struct CoinsTab {
    data: BTreeMap<String, (f64, i32)>,
}

impl Widget for CoinsTab {
    fn render(self, area: Rect, buf: &mut Buffer) {
        self.barchart().render(area, buf);
    }
}

impl CoinsTab {
    pub fn new() -> Self {
        let data = get_coins_data().unwrap();

        Self { data }
    }

    pub fn barchart(&self) -> BarChart<'_> {
        let bars: Vec<Bar> = self.data.iter().map(|(k, v)| bar(k, v.0)).collect();
        BarChart::default()
            .data(BarGroup::default().bars(&bars))
            .block(Block::new())
            .bar_width(8)
    }
}

fn bar(name: &str, value: f64) -> Bar<'_> {
    Bar::default()
        .value(value as u64)
        .label(Line::from(name))
        .text_value(format!("{:.2}", value))
        .style(Color::LightMagenta)
    // .value_style(style)
}
