use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Stylize},
    text::Line,
    widgets::Widget,
};

use crate::tui::{coins_tab::CoinsTab, history_tab::HistoryTab, table_tab::TableTab};

#[derive(Debug, Default, Copy, Clone)]
pub enum RbalTabs {
    #[default]
    Tab1,
    Tab2,
    Tab3,
}

impl RbalTabs {
    /// Get the previous tab, if there is no previous tab return the current tab.
    pub fn previous(self, current: u8) -> Self {
        match current {
            0 => Self::Tab3,
            1 => Self::Tab1,
            2 => Self::Tab2,
            _ => Self::Tab1,
        }
    }

    /// Get the next tab, if there is no next tab return the current tab.
    pub fn next(self, current: u8) -> Self {
        match current {
            0 => Self::Tab2,
            1 => Self::Tab3,
            2 => Self::Tab1,
            _ => Self::Tab1,
        }
    }

    pub fn titles() -> Vec<Line<'static>> {
        vec![
            format!("Tab 1").fg(Color::White).bg(Color::Red).into(),
            format!("Tab 2").fg(Color::White).bg(Color::Green).into(),
            format!("Tab 3").fg(Color::White).bg(Color::Blue).into(),
        ]
    }

    pub fn index(&self) -> u8 {
        match self {
            Self::Tab1 => 0,
            Self::Tab2 => 1,
            Self::Tab3 => 2,
        }
    }
}

impl Widget for RbalTabs {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let tab1 = HistoryTab::new();
        let tab2 = CoinsTab::new();
        let tab3 = TableTab::default();
        match self {
            Self::Tab1 => tab1.render(area, buf),
            Self::Tab2 => tab2.render(area, buf),
            Self::Tab3 => tab3.render(area, buf),
        }
    }
}
