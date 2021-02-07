use tui::{
    widgets::{Block, List, ListItem, Borders, Paragraph},
    layout::{Layout, Constraint, Direction, Margin},
    text::{Span, Spans, Text},
    backend::TermionBackend,
    style::{Color, Modifier, Style},
    Terminal,
};

use std::{
    io,
    fs,
    env, 
    process::Command,
    path::Path,
    fs::File,
};

use termion::{
    raw::IntoRawMode,
    event::Key,
};
mod event;
mod ui;
use event::{Event, Events};

fn main() -> Result<(), io::Error> {
    Command::new("clear").spawn().unwrap();
    let stdout = io::stdout().into_raw_mode()?;
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    let events = Events::new();
    let mut selected_index: u16 = 0;
    let mut help = false;
    let mut current_is_dir: bool;

    loop {
        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .margin(0)
            .constraints([
                Constraint::Percentage(30),
                Constraint::Percentage(50),
                Constraint::Percentage(20),
            ].as_ref())
            .split(terminal.get_frame().size());
         
        let current_dir = env::current_dir().unwrap();
        let current_dir = current_dir.to_str().unwrap();
        
        let dir_contents = fs::read_dir(current_dir).unwrap();
        
        let mut dir_contents: Vec<String> = dir_contents.map(|key| {
            format!("{}", key.unwrap()
                    .path()
                    .to_str()
                    .unwrap()
            )
        }).collect();
        dir_contents.sort();
        
        let dir_contents_length = dir_contents.len();
        
        let current_file = &dir_contents[selected_index as usize].clone();
        
        let file_contents: String;

        let mut selected_dir_contents: List = List::new(vec![ListItem::new(String::new())]);
        
        let file_details: String;
        
        let mut to_replace = current_dir.to_owned();
        to_replace.push('/');
        
        let file_as_path = Path::new(&current_file);
        if file_as_path.is_dir() {
            current_is_dir = true;
            let metadata = file_as_path.metadata().unwrap();
            
            file_details = format!(
"Is directory: true
Read-only: {}
Time since modification: {:?}
Time since accessed: {:?}",
                metadata.permissions().readonly(), 
                metadata.modified().unwrap().elapsed().unwrap(),
                metadata.accessed().unwrap().elapsed().unwrap()
            );
            let mut to_replace_temp = current_file.clone();
            
            to_replace_temp.push('/');
            
            let selected_dir;
            
            if let Err(err) = fs::read_dir(&current_file) {
                file_contents = err.to_string();
            } else { 
                current_is_dir = true;
                selected_dir = fs::read_dir(&current_file).unwrap();
                file_contents = String::new();
                
                let tmp1: Vec<String> = selected_dir.map(|key| {
                    format!("{}", key.unwrap()
                        .path()
                        .to_str()
                        .unwrap()
                     )
                }).collect();
                let selected_dir: Vec<String> = tmp1.iter().map(|entry| {
                    entry.to_string().replace(&to_replace[..], "")
                }).collect();
                let selected_dir: Vec<ListItem> = selected_dir.iter().map(|o| {
                    let content = vec![Spans::from(Span::from(format!("{}", o)))];
                    let item = ListItem::new(content);
                    let res;
                    if Path::new(&o).is_dir() { res = item.style(Style::default().add_modifier(Modifier::BOLD).fg(Color::Blue)); }
                    else { res = item.style(Style::default()); }
                    res
                }).collect();
                
                selected_dir_contents = List::new(selected_dir);
            }
        } else {
            current_is_dir = false;

            selected_dir_contents = List::new(vec![ListItem::new(String::new())]);

            let path = &dir_contents[selected_index as usize];
            let file = File::open(path).unwrap();
            
            let metadata = file.metadata().unwrap();
            file_details = format!(
"Is directory: {} 
Read-only: {}
Time since modification: {:?}
Time since accessed: {:?}", 
                metadata.is_dir(), 
                metadata.permissions().readonly(),
                metadata.modified().unwrap().elapsed().unwrap(),
                metadata.accessed().unwrap().elapsed().unwrap()
            );
            
            let test = fs::read_to_string(&dir_contents[selected_index as usize]);
            
            if let Err(err) = test {
                file_contents = err.to_string();
            } else {
                file_contents = test.unwrap();
            }
        }
        
        let dir_contents: Vec<String> = dir_contents.iter().map(|entry| {
            entry.to_string().replace(&to_replace[..], "")
        }).collect();
        
        let dir_contents: Vec<ListItem> = dir_contents.iter()
            .map(|o|{
                let content = vec![Spans::from(Span::from(format!("{}", o)))];
                let item = ListItem::new(content);
                let res;
                if Path::new(o).is_dir() { res = item.style(Style::default().add_modifier(Modifier::BOLD).fg(Color::Blue)); }
                else { res = item.style(Style::default()); }
                res
            }).collect();
            
        let dir_contents = List::new(dir_contents).block(Block::default());
        let file_details = Paragraph::new(Text::from(file_details))
            .style(Style::default())
            .block(Block::default()
            .borders(Borders::ALL)
            .title("File Details"));
         
        terminal.draw(|f| {
            let dir_contents_pos = chunks[0].inner(&Margin { horizontal: 0, vertical: 0 });
            let file_contents_pos = chunks[1].inner(&Margin { horizontal: 0, vertical: 0 });
            let file_details_pos = chunks[2].inner(&Margin { horizontal: 0, vertical: 0 });
            
            let dir_contents = dir_contents
                .style(Style::default())
                .block(Block::default()
                .borders(Borders::ALL)
                .title(current_dir));
            if current_is_dir {
                f.render_widget(selected_dir_contents, file_contents_pos);
            } else {
                let file_contents = Paragraph::new(Text::from(file_contents))
                    .style(Style::default())
                    .block(Block::default()
                    .borders(Borders::ALL)
                    .title(&current_file[..]));
                f.render_widget(file_contents, file_contents_pos);
            }
                
            f.render_widget(dir_contents, dir_contents_pos);
            if help {
                let help_message = Paragraph::new(Text::from(ui::help_message()))
                    .block(Block::default()
                           .style(Style::default())
                           .borders(Borders::ALL)
                           .title("Help"));
                f.render_widget(help_message, file_details_pos);
            } else {
                f.render_widget(file_details, file_details_pos);
            }
            
            f.set_cursor(dir_contents_pos.x + 1, dir_contents_pos.y + selected_index + 1);
            
        }).unwrap();
        if let Event::Input(input) = events.next().unwrap() {
            match input {
                Key::Up => {
                    selected_index = if selected_index == 0 { selected_index } else { selected_index - 1 };
                }
                
                Key::Down => {
                    selected_index = if selected_index == dir_contents_length as u16 - 1 { selected_index } else { selected_index + 1 };
                }
                
                Key::Left => {
                    let mut tmp = String::from(current_dir);
                    while tmp.as_bytes()[tmp.len() - 1] as char != '/' { tmp.pop(); }
                    env::set_current_dir(tmp).unwrap();
                }
                
                Key::Char('\n') => {
                    if Path::new(&current_file[..]).is_dir() {
                        selected_index = 0;
                        std::env::set_current_dir(&current_file[..]).unwrap();
                    }
                }
                
                Key::Char('q') => {
                    break;
                }
                Key::Char('h') => {
                    help = if help { false } else { true };
                }
                
                _ => {}
            }
        }
    }
    terminal.clear().unwrap();
    Ok(())
}
