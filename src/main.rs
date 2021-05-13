use tui::{
    backend::TermionBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Span, Spans, Text},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Terminal,
};

use std::{env, io, path::Path, process::Command};

use termion::{event::Key, raw::IntoRawMode};

use event::{Event, Events};

mod event;

mod config;
mod fs;
mod ui;

mod app;
use app::{App, InputMode};

fn main() -> Result<(), io::Error> {
    Command::new("clear").spawn().unwrap();

    let mut app = App::default();
    let stdout = io::stdout().into_raw_mode()?;
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let events = Events::new();

    terminal.hide_cursor().unwrap();

    loop {
        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .margin(0)
            .constraints(
                [
                    Constraint::Percentage(33),
                    Constraint::Percentage(34),
                    Constraint::Percentage(33),
                ]
                .as_ref(),
            )
            .split(terminal.get_frame().size());

        let current_dir = env::current_dir().unwrap();
        let current_dir = current_dir.to_str().unwrap();

        app.set_dir(fs::dir::read(current_dir));

        if !(&app.current_dir()).is_empty() {
            app.set_file(app.current_dir()[app.selected() as usize].clone());
        }

        let mut selected_dir_contents: List = List::new(vec![ListItem::new(String::new())])
            .style(Style::default())
            .block(
                Block::default().borders(Borders::ALL).title(Span::styled(
                    &app.current_file()[..],
                    Style::default()
                        .add_modifier(Modifier::BOLD)
                        .fg(config::title_color()),
                )),
            );

        let file_as_path = Path::new(app.current_file());

        if file_as_path.is_dir() {
            let mut to_replace_temp = app.current_file().clone();

            to_replace_temp.push('/');

            let mut selected_dir_formatted: Vec<String> = fs::dir::read(&app.current_file());
            selected_dir_formatted.sort();

            let selected_dir_list: Vec<ListItem> = selected_dir_formatted
                .iter()
                .map(|o| {
                    let content = vec![Spans::from(Span::from(format!("{}", o)))];

                    let mut item = ListItem::new(content);

                    let mut temp_path = to_replace_temp.clone();
                    temp_path.push_str(o);

                    if Path::new(&temp_path).is_dir() {
                        item = item.style(
                            Style::default()
                                .add_modifier(Modifier::BOLD)
                                .fg(config::directory_color()),
                        );
                    } else {
                        item = item.style(Style::default());
                    }
                    item
                })
                .collect();

            selected_dir_contents = List::new(selected_dir_list).block(
                Block::default().borders(Borders::ALL).title(Span::styled(
                    &app.current_file()[..],
                    Style::default()
                        .add_modifier(Modifier::BOLD)
                        .fg(config::title_color()),
                )),
            );
        }

        let dir_widget: Vec<ListItem> = app.current_dir()
            .iter()
            .map(|entry| {
                let content = vec![Spans::from(Span::from(entry.to_string()))];

                let item;
                let mut style = Style::from(Style::default());

                if Path::new(entry).is_dir() {
                    style = Style::default()
                        .add_modifier(Modifier::BOLD)
                        .fg(config::directory_color());
                }

                if &app.current_file()[..] == entry {
                    style = Style::default()
                        .add_modifier(Modifier::BOLD)
                        .fg(config::selected_file_color());
                }

                match app.input_mode {
                    InputMode::Rename => {
                        if &app.current_file()[..] == entry {
                            item = ListItem::new(if !&app.input_string[..].is_empty() {
                                &app.input_string[..]
                            } else {
                                "\n"
                            });
                            style = style.add_modifier(Modifier::RAPID_BLINK);
                        } else {
                            item = ListItem::new(content);
                        }
                    }

                    InputMode::Normal | InputMode::Create => {
                        item = ListItem::new(content);
                    }

                    InputMode::Delete => {
                        if &app.current_file()[..] == entry {
                            item = ListItem::new(vec![Spans::from(Span::from(format!(
                                "{} [ y/n ]",
                                entry
                            )))]);

                            style = Style::default().add_modifier(Modifier::BOLD).fg(Color::Red);
                        } else {
                            item = ListItem::new(content)
                        };
                    }
                }

                item.style(style)
            })
            .collect();

        let dir_widget = List::new(dir_widget);

        let file_info = Paragraph::new(Text::from(fs::info(&app.current_file()[..])))
            .style(Style::default())
            .block(
                Block::default().borders(Borders::ALL).title(Span::styled(
                    "File Information",
                    Style::default()
                        .add_modifier(Modifier::BOLD)
                        .fg(config::title_color()),
                )),
            );

        terminal
            .draw(|f| {
                let dir_contents_pos = chunks[0];
                let file_contents_pos = chunks[1];
                let file_info_pos = chunks[2];

                let dir_widget = dir_widget.style(Style::default()).block(
                    Block::default().borders(Borders::ALL).title(Span::styled(
                        current_dir,
                        Style::default()
                            .add_modifier(Modifier::BOLD)
                            .fg(config::title_color()),
                    )),
                );

                match app.input_mode {
                    InputMode::Rename => {
                        f.set_cursor(
                            chunks[0].x + app.input_string.len() as u16 + 1,
                            chunks[0].y + app.selected() + 1,
                        );
                    }

                    InputMode::Create => {
                        f.set_cursor(
                            chunks[0].x + app.input_string.len() as u16 + 1,
                            chunks[0].y + app.current_dir().len() as u16 + 1,
                        );

                        let widget = Paragraph::new(Text::from(app.input_string.as_ref())).style(
                            Style::default()
                                .add_modifier(Modifier::BOLD)
                                .fg(Color::Yellow),
                        );

                        f.render_widget(
                            widget,
                            chunks[0].inner(&tui::layout::Margin {
                                horizontal: 1,
                                vertical: app.current_dir().len() as u16 + 1,
                            }),
                        );
                    }
                    _ => {}
                }

                if Path::new(&app.current_file()[..]).is_dir() {
                    f.render_widget(selected_dir_contents, file_contents_pos);
                } else {
                    let file_contents =
                        Paragraph::new(Text::from(fs::file::read(&app.current_file()[..])))
                            .style(Style::default())
                            .block(
                                Block::default().borders(Borders::ALL).title(Span::styled(
                                    &app.current_file()[..],
                                    Style::default()
                                        .add_modifier(Modifier::BOLD)
                                        .fg(config::title_color()),
                                )),
                            ).scroll((app.scroll_offset, 0));

                    f.render_widget(file_contents, file_contents_pos);
                }

                f.render_widget(dir_widget, dir_contents_pos);

                if app.help {
                    let help_message = Paragraph::new(Text::from(ui::help_message())).block(
                        Block::default()
                            .style(Style::default())
                            .borders(Borders::ALL)
                            .title(Span::styled(
                                "Help",
                                Style::default()
                                    .add_modifier(Modifier::BOLD)
                                    .fg(config::title_color()),
                            )),
                    );

                    f.render_widget(help_message, file_info_pos);
                } else {
                    f.render_widget(file_info, file_info_pos);
                }
            })
            .unwrap();

        if let Event::Input(input) = events.next().unwrap() {
            match input {
                Key::Alt(c) => {
                    match c {
                        'j' => app.scroll_offset += 1,
                        'k' => if app.scroll_offset > 0 { app.scroll_offset -= 1 },
                        _ => ()
                    }
                }
                Key::Up => match app.input_mode {
                    InputMode::Create => {}
                    _ => {
                        app.move_up();
                    }
                },

                Key::Down => {
                    if !app.current_dir().is_empty() {
                        match app.input_mode {
                            InputMode::Create => {}
                            _ => {
                                app.move_down();
                            }
                        };
                    }
                }

                Key::Left => {
                    app.set_selected(0);

                    let mut tmp = String::from(current_dir);
                    while tmp.as_bytes()[tmp.len() - 1] as char != '/' {
                        tmp.pop();
                    }

                    env::set_current_dir(tmp).unwrap();
                }

                Key::Right => {
                    if Path::new(&app.current_file()[..]).is_dir() {
                        let set_dir = std::env::set_current_dir(&app.current_file()[..]);
                        if let Ok(()) = set_dir {
                            app.set_selected(0);
                            set_dir.unwrap();
                        }
                    }
                }

                Key::Char('\n') => match app.input_mode {
                    InputMode::Normal => {
                        if Path::new(&app.current_file()[..]).is_dir() {
                            let set_dir = std::env::set_current_dir(&app.current_file()[..]);

                            if let Ok(()) = set_dir {
                                app.set_selected(0);
                                set_dir.unwrap();
                            }
                        }
                    }
                    InputMode::Rename => {
                        app.input_mode = InputMode::Normal;
                        fs::rename(&app.current_file()[..], &app.input_string[..]).unwrap();
                        app.input_string.clear();
                    }
                    InputMode::Delete => match &app.input_string[..] {
                        _ => {
                            app.input_string.clear();
                            app.input_mode = InputMode::Normal;
                        }
                    },
                    InputMode::Create => {
                        let file = fs::file::create(&app.input_string[..]);
                        if let Ok(_) = file {
                            file.unwrap();
                        }
                        app.input_mode = InputMode::Normal;
                    }
                },

                Key::Backspace => match app.input_mode {
                    InputMode::Rename => {
                        app.input_string.pop();
                    }
                    _ => {}
                },

                Key::Esc => match app.input_mode {
                    InputMode::Normal => {}
                    InputMode::Rename | InputMode::Delete | InputMode::Create => {
                        app.input_mode = InputMode::Normal;
                        app.input_string.clear();
                    }
                },
                Key::Char(c) => match app.input_mode {
                    InputMode::Rename | InputMode::Create => {
                        app.input_string.push(c);
                    }

                    InputMode::Delete => match c {
                        'y' => {
                            fs::delete(&app.current_file()[..]).unwrap();
                            app.input_string.clear();
                            app.input_mode = InputMode::Normal;
                            if app.selected() >= app.current_dir().len() as u16 - 1 {
                                app.set_selected(app.current_dir().len() as u16 - 2);
                            }
                        }
                        _ => {
                            app.input_mode = InputMode::Normal;
                            app.input_string.clear();
                        }
                    },

                    InputMode::Normal => match c {
                        'q' => {
                            break;
                        }
                        'h' => {
                            app.help = !app.help;
                        }
                        'r' => {
                            app.input_mode = InputMode::Rename;
                        }
                        'd' => {
                            app.input_mode = InputMode::Delete;
                        }
                        'c' => {
                            app.input_mode = InputMode::Create;
                        }
                        _ => {}
                    },
                },
                _ => {}
            }
        }
    }
    terminal.clear().unwrap();
    Ok(())
}
