use ratatui::style::Color;

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
            foreground: Color::White,
            background: Color::Rgb(54, 38, 57),
            alt_bg: Color::Rgb(48, 29, 52),
            header_fg: Color::Rgb(57, 204, 241),
            header_bg: Color::Rgb(21, 2, 33),
        }
    }
}
