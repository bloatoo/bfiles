pub enum InputMode {
    Normal,
    Rename,
    Delete,
}
pub struct App {
    pub input_mode: InputMode,
    pub input_string: String,
}
impl App {
    pub fn new() -> App {
        App {
            input_mode: InputMode::Normal,
            input_string: String::new(),
        }
    }
}
