use tui::style::Color;

pub struct Config {
    pub title_color: Color,
    pub selected_file_color: Color,
    pub directory_color: Color
}

impl Config {
    pub fn default() -> Self {
        Self {
            title_color: Color::Magenta,
            selected_file_color: Color::Green,
            directory_color: Color::Blue
        }
    }
}
