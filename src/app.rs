pub enum InputMode {
    Normal,
    Rename,
    Delete,
    Create,
}
pub struct App {
    pub input_mode: InputMode,
    pub input_string: String,
}
impl App {
    pub fn default() -> App {
        App {
            input_mode: InputMode::Normal,
            input_string: String::new(),
        }
    }
}
