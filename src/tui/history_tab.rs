use anyhow::Result;
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Style, Stylize},
    symbols,
    widgets::{Axis, Block, Chart, Dataset, GraphType, LegendPosition, Widget},
};

use crate::db::load_db;

pub struct HistoryTab {
    usd: f64,
    data: Vec<(f64, f64)>,
}

impl HistoryTab {
    pub fn new() -> Self {
        let (usd, data) = get_chart_data().unwrap();

        Self { usd, data }
    }

    fn get_xbounds(&self) -> [f64; 2] {
        let x_min = self
            .data
            .iter()
            .map(|f| f.0)
            .min_by(|a, b| a.total_cmp(b))
            .unwrap();
        let x_max = self
            .data
            .iter()
            .map(|f| f.0)
            .max_by(|a, b| a.total_cmp(b))
            .unwrap();

        [x_min, x_max]
    }

    fn get_ybounds(&self) -> [f64; 2] {
        let y_min = self
            .data
            .iter()
            .map(|f| f.1)
            .min_by(|a, b| a.total_cmp(b))
            .unwrap();
        let y_max = self
            .data
            .iter()
            .map(|f| f.1)
            .max_by(|a, b| a.total_cmp(b))
            .unwrap();

        [y_min, y_max]
    }
}

impl Widget for HistoryTab {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let x_bounds = self.get_xbounds();
        let y_bounds = self.get_ybounds();

        let x_max = x_bounds.last().unwrap().to_owned();
        let usd_data = [(0.0, self.usd), (x_max, self.usd)];

        let dataset = vec![
            Dataset::default()
                .name("USD Spent")
                .marker(symbols::Marker::Braille)
                .style(Style::default().fg(Color::Cyan))
                .graph_type(GraphType::Line)
                .data(&usd_data),
            Dataset::default()
                .name("Crypto Spent")
                .marker(symbols::Marker::Braille)
                .style(Style::default().fg(Color::LightMagenta))
                .graph_type(GraphType::Line)
                .data(&self.data),
        ];

        Chart::new(dataset)
            .block(Block::new())
            .x_axis(
                Axis::default()
                    .title("Transactions")
                    .style(Style::default().gray())
                    .bounds(x_bounds)
                    .labels(get_labels(x_bounds)),
            )
            .y_axis(
                Axis::default()
                    .title("Total Amount")
                    .style(Style::default().gray())
                    .bounds(y_bounds)
                    .labels(get_labels(y_bounds)),
            )
            .legend_position(Some(LegendPosition::Top))
            .render(area, buf);
    }
}

fn get_chart_data() -> Result<(f64, Vec<(f64, f64)>)> {
    let db = load_db();
    let mut stmt = db.prepare("SELECT amount FROM rbal")?;
    let mut rows = stmt.query([])?;

    let mut usd: f64 = 0.0;
    let mut data: Vec<(f64, f64)> = Vec::new();
    let mut x: f64 = 0.0;
    let mut y: f64 = 0.0;

    data.push((x, y));

    while let Some(row) = rows.next()? {
        let amount: f64 = row.get(0)?;

        if amount > 0.0 {
            x += 1.0;
            y += amount;

            data.push((x, y));
        } else {
            usd += amount;
        }
    }

    Ok((usd.abs(), data))
}

fn get_labels(bounds: [f64; 2]) -> Vec<String> {
    let first = bounds.first().unwrap().to_string();
    let half = (bounds.last().unwrap() / 2.0).to_string();
    let last = bounds.last().unwrap().to_string();

    vec![first, half, last]
}
