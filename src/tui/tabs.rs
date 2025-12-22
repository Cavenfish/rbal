use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Stylize},
    text::Line,
    widgets::{StatefulWidget, TableState, Widget},
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
            make_tab_title("Spendings"),
            make_tab_title("Coins"),
            make_tab_title("Table"),
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

impl StatefulWidget for RbalTabs {
    type State = TableState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        let tab1 = HistoryTab::new();
        let tab2 = CoinsTab::new();
        let tab3 = TableTab::default();
        match self {
            Self::Tab1 => tab1.render(area, buf),
            Self::Tab2 => tab2.render(area, buf),
            Self::Tab3 => tab3.render(area, buf, state),
        }
    }
}

// impl Widget for RbalTabs {
//     fn render(self, area: Rect, buf: &mut Buffer) {
//         let tab1 = HistoryTab::new();
//         let tab2 = CoinsTab::new();
//         let tab3 = TableTab::default();
//         let mut table_state = TableState::default().with_selected(0);
//         match self {
//             Self::Tab1 => tab1.render(area, buf),
//             Self::Tab2 => tab2.render(area, buf),
//             Self::Tab3 => tab3.render(area, buf, &mut table_state),
//         }
//     }
// }

fn make_tab_title(title: &str) -> Line<'_> {
    let fg = Color::Rgb(139, 239, 238);
    let bg = Color::Rgb(0, 87, 86);
    format!("{}", title).fg(fg).bg(bg).bold().into()
}
