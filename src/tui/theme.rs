use ratatui::style::Color;

#[derive(Debug)]
pub struct RbalTheme {
    pub foreground: Color,
    pub background: Color,
}

impl Default for RbalTheme {
    fn default() -> Self {
        Self {
            foreground: Color::White,
            background: Color::Black,
        }
    }
}

#[derive(Debug)]
pub struct TableTheme {
    pub foreground: Color,
    pub background: Color,
    pub alt_bg: Color,
    pub header_fg: Color,
    pub header_bg: Color,
}

impl Default for TableTheme {
    fn default() -> Self {
        Self {
            foreground: Color::Black,
            background: Color::Gray,
            alt_bg: Color::DarkGray,
            header_fg: Color::Blue,
            header_bg: Color::Black,
        }
    }
}
