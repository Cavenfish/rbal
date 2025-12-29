use anyhow::Result;
use crossterm::event::{self, KeyCode};
use ratatui::{
    DefaultTerminal,
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    text::Line,
    widgets::{StatefulWidget, TableState, Tabs, Widget},
};
use std::cell::RefCell;

use crate::{args::TransInfo, utils::get_rows};

use super::tabs::RbalTabs;

fn render_footer(area: Rect, buf: &mut Buffer) {
    Line::raw("◄ ► to change tab | Press q to quit")
        .centered()
        .render(area, buf);
}

#[derive(Debug, Default)]
pub struct App {
    pub state: AppState,
    selected_tab: RbalTabs,
    tab_index: u8,
}

impl App {
    pub fn run(mut self, mut terminal: DefaultTerminal) -> Result<()> {
        self.state.running = true;
        while self.state.running {
            terminal.draw(|frame| {
                frame.render_widget(&self, frame.area());
            })?;
            if let Some(key) = event::read()?.as_key_press_event() {
                match key.code {
                    KeyCode::Char('d') | KeyCode::Right => self.next_tab(),
                    KeyCode::Char('a') | KeyCode::Left => self.previous_tab(),
                    KeyCode::Char('q') | KeyCode::Esc => self.quit(),
                    KeyCode::Down => self.state.next_row(),
                    KeyCode::Up => self.state.previous_row(),
                    _ => {}
                }
            }
        }
        Ok(())
    }

    fn render_tabs(&self, area: Rect, buf: &mut Buffer) {
        let titles = RbalTabs::titles();
        let selected_tab_index = self.selected_tab.index() as usize;
        Tabs::new(titles)
            .select(selected_tab_index)
            .padding("", "")
            .divider(" ")
            .render(area, buf);
    }

    pub fn next_tab(&mut self) {
        self.selected_tab = self.selected_tab.next(self.tab_index);
        self.tab_index = match self.tab_index {
            0 => 1,
            1 => 2,
            2 => 0,
            _ => 0,
        };
    }

    pub fn previous_tab(&mut self) {
        self.selected_tab = self.selected_tab.previous(self.tab_index);
        self.tab_index = match self.tab_index {
            0 => 2,
            1 => 0,
            2 => 1,
            _ => 0,
        };
    }

    fn quit(&mut self) {
        self.state.running = false;
    }
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        use Constraint::{Length, Min};
        let vertical = Layout::vertical([Length(1), Min(0), Length(1)]);
        let [header_area, inner_area, footer_area] = vertical.areas(area);

        let horizontal = Layout::horizontal([Min(0)]);
        let [tabs_area] = horizontal.areas(header_area);

        self.render_tabs(tabs_area, buf);
        self.selected_tab
            .render(inner_area, buf, &mut self.state.table_state.borrow_mut());
        render_footer(footer_area, buf);
    }
}

#[derive(Debug)]
pub struct AppState {
    running: bool,
    table_state: RefCell<TableState>,
    table_items: Vec<TransInfo>,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            running: true,
            table_state: RefCell::new(TableState::default().with_selected(0)),
            table_items: get_rows(),
        }
    }
}

impl AppState {
    pub fn next_row(&mut self) {
        let i = match self.table_state.borrow().selected() {
            Some(i) => {
                if i >= self.table_items.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.table_state.borrow_mut().select(Some(i));
    }

    pub fn previous_row(&mut self) {
        let i = match self.table_state.borrow().selected() {
            Some(i) => {
                if i == 0 {
                    self.table_items.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.table_state.borrow_mut().select(Some(i));
    }
}
