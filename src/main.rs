use tui::{
    widgets::{Block, List, ListItem, Borders, Paragraph},
    layout::{Layout, Constraint, Direction},
    text::{Span, Spans, Text},
    backend::TermionBackend,
    style::{Modifier, Style},
    Terminal,
};

use std::{
    io,
    env, 
    process::Command,
    path::Path,
};

use termion::{
    raw::IntoRawMode,
    event::Key,
};

use event::{Event, Events};
mod event;
mod ui;
mod config;
mod fs;

fn main() -> Result<(), io::Error> {
    Command::new("clear").spawn().unwrap();
    let stdout = io::stdout().into_raw_mode()?;
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    let events = Events::new();
    let mut selected_index: u16 = 0;
    let mut help = false;
    terminal.hide_cursor().unwrap();

    loop {
        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .margin(0)
            .constraints([
                Constraint::Percentage(33),
                Constraint::Percentage(34),
                Constraint::Percentage(33),
            ].as_ref())
            .split(terminal.get_frame().size());
         
        let current_dir = env::current_dir().unwrap();
        let current_dir = current_dir.to_str().unwrap();
        
        let dir_contents = fs::dir::read(current_dir);
        
        let dir_contents_length = dir_contents.len();
        
        let mut current_file = String::new();
        
        if !(&dir_contents).is_empty() {
            current_file = (&dir_contents)[selected_index as usize].clone();
        }
        
        let mut selected_dir_contents: List = List::new(vec![ListItem::new(String::new())])
            .style(Style::default())
            .block(Block::default()
                .borders(Borders::ALL)
                .title(Span::styled(&current_file[..], Style::default()
                                    .add_modifier(Modifier::BOLD)
                                    .fg(config::title_color()))));
                
        let mut to_replace = current_dir.to_owned();
        to_replace.push('/');
        
        let file_as_path = Path::new(&current_file);

        if file_as_path.is_dir() {
            
            let mut to_replace_temp = current_file.clone();
            
            to_replace_temp.push('/');
                
                let mut selected_dir_formatted: Vec<String> = fs::dir::read(&current_file).iter().map(|entry| {
                    entry.to_string().replace(&to_replace_temp[..], "")
                }).collect();
                
                selected_dir_formatted.sort();
                
                let selected_dir_list: Vec<ListItem> = selected_dir_formatted.iter().map(|o| {
                    let content = vec![Spans::from(Span::from(format!("{}", o)))];
                    
                    let mut item = ListItem::new(content);
                    
                    let mut temp_path = to_replace_temp.clone();
                    temp_path.push_str(o);
                    
                    if Path::new(&temp_path).is_dir() {
                        item = item.style(Style::default()
                                        .add_modifier(Modifier::BOLD)
                                        .fg(config::directory_color())); 
                    } else { 
                        item = item.style(Style::default()); 
                    }
                    item
                }).collect();
                
                selected_dir_contents = List::new(selected_dir_list)
                    .block(Block::default()
                    .borders(Borders::ALL)
                    .title(Span::styled(&current_file[..], Style::default()
                                        .add_modifier(Modifier::BOLD)
                                        .fg(config::title_color()))));
        }
       
        let dir_widget: Vec<ListItem> = dir_contents.iter()
            .map(|entry|{
                let content = vec![Spans::from(Span::from(format!("{}", entry
                                                                  .replace(&to_replace[..], ""))))];
                let mut item = ListItem::new(content);
                    
                if Path::new(entry).is_dir() { 
                    item = item.style(Style::default()
                                    .add_modifier(Modifier::BOLD).fg(config::directory_color())); 
                } else { 
                    item = item.style(Style::default()); 
                }
                if &current_file[..] == entry {
                    item = item.style(Style::default()
                                      .add_modifier(Modifier::BOLD)
                                      .fg(config::selected_file_color()));
                }
                item
            }).collect();
            
        let dir_widget = List::new(dir_widget);

        let file_details = Paragraph::new(Text::from(fs::details(&current_file[..])))
            .style(Style::default())
            .block(Block::default()
            .borders(Borders::ALL)
            .title(Span::styled("File Details", Style::default()
                                .add_modifier(Modifier::BOLD)
                                .fg(config::title_color()))));
                                    
        terminal.draw(|f| {
            let dir_contents_pos = chunks[0];
            let file_contents_pos = chunks[1];
            let file_details_pos = chunks[2];
            
            let dir_widget = dir_widget
                .style(Style::default())
                .block(Block::default()
                .borders(Borders::ALL)
                .title(Span::styled(current_dir, Style::default()
                                    .add_modifier(Modifier::BOLD)
                                    .fg(config::title_color()))));
                
            if Path::new(&current_file[..]).is_dir() {
                f.render_widget(selected_dir_contents, file_contents_pos);
            } else {
                let file_contents = Paragraph::new(Text::from(fs::file::read(&current_file[..])))
                    .style(Style::default())
                    .block(Block::default()
                    .borders(Borders::ALL)
                    .title(Span::styled(&current_file[..], Style::default()
                                        .add_modifier(Modifier::BOLD)
                                        .fg(config::title_color()))));
                                        
                f.render_widget(file_contents, file_contents_pos);
            }
         
            f.render_widget(dir_widget, dir_contents_pos);
            
            if help {
                let help_message = Paragraph::new(Text::from(ui::help_message()))
                    .block(Block::default()
                           .style(Style::default())
                           .borders(Borders::ALL)
                           .title(Span::styled("Help", Style::default()
                                               .add_modifier(Modifier::BOLD)
                                               .fg(config::title_color()))));
                    
                f.render_widget(help_message, file_details_pos);
            } else {
                f.render_widget(file_details, file_details_pos);
            }
        }).unwrap();
        
        if let Event::Input(input) = events.next().unwrap() {
            match input {
                Key::Up => {
                    selected_index = if selected_index == 0 {
                        selected_index
                    } else {
                        selected_index - 1
                    };
                }
                
                Key::Down => {
                    if !dir_contents.is_empty() {
                        selected_index = if selected_index == dir_contents_length as u16 - 1 {
                            selected_index 
                        } else { 
                            selected_index + 1 
                        };
                    }
                }
                
                Key::Left => {
                    selected_index = 0;
                    
                    let mut tmp = String::from(current_dir);
                    while tmp.as_bytes()[tmp.len() - 1] as char != '/' { tmp.pop(); }
                    
                    env::set_current_dir(tmp).unwrap();
                }
                Key::Right => {
                    if Path::new(&current_file[..]).is_dir() {
                        let set_dir = std::env::set_current_dir(&current_file[..]);
                        if let Ok(()) = set_dir {
                            selected_index = 0;
                            set_dir.unwrap();
                        } 
                    }
                }
                
                Key::Char('\n') => {
                    if Path::new(&current_file[..]).is_dir() {
                        let set_dir = std::env::set_current_dir(&current_file[..]);
                        
                        if let Ok(()) = set_dir {
                            selected_index = 0;
                            set_dir.unwrap();
                        } 
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
