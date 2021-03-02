use std::cmp::Ordering;

pub enum InputMode {
    Normal,
    Rename,
    Delete,
    Create,
}
pub struct App {
    pub input_mode: InputMode,
    pub input_string: String,
    pub help: bool,
    selected_index: u16,
    dir: Vec<String>,
    selected_file: String,
}
impl App {
    pub fn default() -> App {
        App {
            input_mode: InputMode::Normal,
            input_string: String::new(),
            help: false,
            selected_index: 0,
            dir: vec![],
            selected_file: String::new()
        }
    }
    pub fn selected(&self) -> u16 {
        self.selected_index
    }
    
    pub fn move_up(&mut self) {
        self.selected_index = match self.selected_index.cmp(&0) {
            Ordering::Equal => self.selected_index,
            _ => self.selected_index - 1,
        };
    }
    
    pub fn move_down(&mut self) {

        self.selected_index = match self.selected_index.cmp(&((&self.dir.len() - 1) as u16)) {
            Ordering::Equal => {
                    self.selected_index
            },
            _ => {
                self.selected_index + 1
            }
        }
    }
    pub fn set_selected(&mut self, index: u16) {
        self.selected_index = index;
    }

    pub fn current_dir(&self) -> &Vec<String> {
        &self.dir
    }

    pub fn set_dir(&mut self, dir: Vec<String>) {
        self.dir = dir;
    }

    pub fn current_file(&self) -> &String {
        &self.selected_file
    }
    
    pub fn set_file(&mut self, file: String) {
        self.selected_file = file;
    }
}
