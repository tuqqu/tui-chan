#![allow(clippy::single_match)]

use std::{env, io, process, str};

use client::ChanClient;
use clipboard::{ClipboardContext, ClipboardProvider};
use open::that as open_in_browser;
use reqwest::Client;
use termion::event::Key;
use termion::input::MouseTerminal;
use termion::raw::IntoRawMode;
use termion::screen::AlternateScreen;
use tokio::runtime::Runtime;
use tui::backend::TermionBackend;
use tui::layout::{Constraint, Direction, Layout};
use tui::style::{Color, Modifier, Style};
use tui::text::{Span, Spans};
use tui::widgets::{Block, Borders, List, ListItem, Paragraph, Wrap};
use tui::Terminal;

use crate::app::App;
use crate::client::api::{
    from_name as channel_provider_from_name, ChannelProvider, ContentUrlProvider,
};
use crate::event::{Event, Events};
use crate::format::{format_default, format_post_full, format_post_short};
use crate::model::{Board, Thread, ThreadList, ThreadPost};
use crate::style::{SelectedField, StyleProvider};

mod app;
mod client;
mod event;
mod format;
mod model;
mod style;

fn main() -> Result<(), io::Error> {
    let stdout = io::stdout().into_raw_mode()?;
    let stdout = MouseTerminal::from(stdout);
    let stdout = AlternateScreen::from(stdout);
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    let mut runtime = Runtime::new()?;

    let args: Vec<String> = env::args().collect();
    let chan: &str = if args.len() == 1 { "default" } else { &args[1] };

    let api: &dyn ChannelProvider = match channel_provider_from_name(chan) {
        Some(api) => api,
        None => {
            println!("Imageboard name \"{}\" is not valid.", chan);
            process::exit(1);
        }
    };

    let client = ChanClient::new(Client::new(), api.as_api());
    let events = Events::new();
    let api: &dyn ContentUrlProvider = api.as_content();

    let mut boards: Vec<Board> = vec![];
    runtime.block_on(async {
        boards = client.get_boards().await.unwrap();
    });

    let mut app = App::new(boards, vec![], vec![]);
    app.set_shown_board_list(true);
    let mut selected_field: SelectedField = SelectedField::BoardList;
    let mut thread_list = ThreadList::new();
    let style_prov = StyleProvider::new();
    let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();

    loop {
        terminal.draw(|f| {
            let block_style = style_prov.default_from_selected_field(&selected_field);
            let scr_share = app.calc_screen_share();

            let mut constraints = vec![Constraint::Min(0)];
            if app.help_bar().shown() {
                constraints.push(Constraint::Length(10));
            }

            let helpbar_chunk = Layout::default()
                .constraints(constraints.as_ref())
                .split(f.size());

            if app.help_bar().shown() {
                let block = Block::default().borders(Borders::NONE).title(Span::styled(
                    app.help_bar().title(),
                    Style::default()
                        .fg(Color::Magenta)
                        .add_modifier(Modifier::BOLD),
                ));
                let paragraph = Paragraph::new(app.help_bar().text())
                    .block(block)
                    .wrap(Wrap { trim: true });
                f.render_widget(paragraph, helpbar_chunk[1]);
            }

            let chunks = Layout::default()
                .direction(Direction::Horizontal)
                .constraints(
                    [
                        Constraint::Percentage(scr_share.board_list()),
                        Constraint::Percentage(scr_share.thread_list()),
                        Constraint::Percentage(scr_share.thread()),
                    ]
                    .as_ref(),
                )
                .split(helpbar_chunk[0]);

            let items: Vec<ListItem> = app
                .boards
                .items
                .iter()
                .map(|board| {
                    let lines = vec![Spans::from(vec![
                        Span::styled(
                            format_default(&format!("/{}/", board.board())),
                            Style::default().fg(Color::Magenta),
                        ),
                        Span::raw(format_default(board.title())),
                    ])];

                    ListItem::new(lines).style(Style::default())
                })
                .collect();

            let items = List::new(items)
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .border_style(Style::default().fg(block_style.border_color().board_list()))
                        .border_type(block_style.border_type().board_list())
                        .title(format_default("Boards ")),
                )
                .highlight_style(
                    Style::default()
                        .bg(*style_prov.highlight_color())
                        .add_modifier(Modifier::BOLD),
                );

            f.render_stateful_widget(items, chunks[0], &mut app.boards.state);

            let thread_len = app.threads.items.len();
            let threads: Vec<ListItem> = app
                .threads
                .items
                .iter()
                .enumerate()
                .map(|(i, thread)| {
                    format_post_short(thread.posts().first().unwrap(), i + 1, thread_len)
                })
                .collect();

            let threads = List::new(threads)
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .border_style(Style::default().fg(block_style.border_color().thread_list()))
                        .border_type(block_style.border_type().thread_list())
                        .title(format_default(&format!(
                            "Threads, page {} {}",
                            thread_list.cur_page(),
                            thread_list.description(),
                        ))),
                )
                .highlight_style(Style::default().bg(*style_prov.highlight_color()));
            f.render_stateful_widget(threads, chunks[1], &mut app.threads.state);

            let thread: Vec<ListItem> = app
                .thread
                .items
                .iter()
                .enumerate()
                .map(|(i, post)| format_post_full(post, i + 1))
                .collect();

            let thread = List::new(thread)
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .border_style(Style::default().fg(block_style.border_color().thread()))
                        .border_type(block_style.border_type().thread())
                        .title(format_default("Thread ")),
                )
                .highlight_style(Style::default().bg(*style_prov.highlight_color()));
            f.render_stateful_widget(thread, chunks[2], &mut app.thread.state);
        })?;

        match events.next().unwrap() {
            Event::Input(input) => match input {
                Key::Char('q') => {
                    break;
                }
                Key::Left | Key::Char('a') => {
                    match selected_field {
                        SelectedField::BoardList => {}
                        SelectedField::ThreadList => {
                            app.set_shown_board_list(true);
                            app.set_shown_thread(false);
                            selected_field = SelectedField::BoardList;
                        }
                        SelectedField::Thread => {
                            app.set_shown_board_list(true);
                            app.set_shown_thread_list(true);
                            app.set_shown_thread(false);
                            selected_field = SelectedField::ThreadList;
                        }
                    };
                }
                Key::Down | Key::Char('s') => {
                    const STEPS: isize = 1;
                    app.advance(&selected_field, STEPS);
                }
                Key::Up | Key::Char('w') => {
                    const STEPS: isize = -1;
                    app.advance(&selected_field, STEPS);
                }
                Key::Ctrl('s') => {
                    const STEPS: isize = 5;
                    app.advance(&selected_field, STEPS);
                }
                Key::Ctrl('w') => {
                    const STEPS: isize = -5;
                    app.advance(&selected_field, STEPS);
                }
                Key::Char('z') => {
                    match selected_field {
                        SelectedField::BoardList => {
                            app.toggle_shown_board_list();
                            selected_field = SelectedField::ThreadList;
                        }
                        SelectedField::ThreadList => {
                            if app.shown_thread() {
                                app.toggle_shown_thread_list();
                                selected_field = SelectedField::Thread;
                            } else {
                                app.toggle_shown_board_list();
                                selected_field = SelectedField::ThreadList;
                            }
                        }
                        SelectedField::Thread => {
                            app.toggle_shown_thread_list();
                            selected_field = SelectedField::Thread;
                        }
                    };
                }
                Key::Char('h') => {
                    app.help_bar_mut().toggle_shown();
                }
                Key::Char('o') => {
                    let url = match selected_field {
                        SelectedField::BoardList => app.url_boards(api),
                        SelectedField::ThreadList => app.url_threads(api),
                        SelectedField::Thread => app.url_thread(api),
                    };

                    open_in_browser(url).expect("Browser error.");
                }
                Key::Ctrl('o') => {
                    let url = match selected_field {
                        SelectedField::BoardList => None,
                        SelectedField::ThreadList => app.media_url_threads(api),
                        SelectedField::Thread => app.media_url_thread(api),
                    };

                    if let Some(url) = url {
                        open_in_browser(url).expect("Browser error.");
                    }
                }
                Key::Char('c') => {
                    let url = match selected_field {
                        SelectedField::BoardList => app.url_boards(api),
                        SelectedField::ThreadList => app.url_threads(api),
                        SelectedField::Thread => app.url_thread(api),
                    };

                    ctx.set_contents(url).expect("Clipboard error.");
                }
                Key::Ctrl('c') => {
                    let url = match selected_field {
                        SelectedField::BoardList => None,
                        SelectedField::ThreadList => app.media_url_threads(api),
                        SelectedField::Thread => app.media_url_thread(api),
                    };

                    if let Some(url) = url {
                        ctx.set_contents(url).expect("Clipboard error.");
                    }
                }
                Key::Char('p') => {
                    match selected_field {
                        SelectedField::ThreadList => {
                            let mut threads: Vec<Thread> = vec![];
                            runtime.block_on(async {
                                let result = client
                                    .get_threads(
                                        app.selected_board().board(),
                                        thread_list.next_page(app.selected_board()),
                                    )
                                    .await;
                                match result {
                                    Ok(data) => threads = data,
                                    Err(err) => eprintln!("{:#?}", err),
                                };

                                app.fill_threads(threads);
                            });
                        }
                        _ => {}
                    };
                }
                Key::Ctrl('p') => {
                    match selected_field {
                        SelectedField::ThreadList => {
                            let mut threads: Vec<Thread> = vec![];
                            runtime.block_on(async {
                                let result = client
                                    .get_threads(
                                        app.selected_board().board(),
                                        thread_list.prev_page(app.selected_board()),
                                    )
                                    .await;
                                match result {
                                    Ok(data) => threads = data,
                                    Err(err) => eprintln!("{:#?}", err),
                                };

                                app.fill_threads(threads);
                            });
                        }
                        _ => {}
                    };
                }
                Key::Char('r') => {
                    match selected_field {
                        SelectedField::ThreadList => {
                            let mut threads: Vec<Thread> = vec![];
                            runtime.block_on(async {
                                let result = client
                                    .get_threads(
                                        app.selected_board().board(),
                                        thread_list.cur_page(),
                                    )
                                    .await;
                                match result {
                                    Ok(data) => threads = data,
                                    Err(err) => eprintln!("{:#?}", err),
                                };

                                app.fill_threads(threads);
                                app.threads.advance_by(1);
                            });
                        }
                        SelectedField::Thread => {
                            let mut thread: Vec<ThreadPost> = vec![];
                            runtime.block_on(async {
                                let result = client
                                    .get_thread(
                                        app.selected_board().board(),
                                        app.selected_thread().posts().first().unwrap().no() as u64,
                                    )
                                    .await;
                                match result {
                                    Ok(data) => thread = data,
                                    Err(err) => eprintln!("{:#?}", err),
                                };

                                app.fill_thread(thread);
                                app.thread.advance_by(1);
                            });
                        }
                        _ => {}
                    };
                }
                Key::Right | Key::Char('d') => {
                    match selected_field {
                        SelectedField::BoardList => {
                            selected_field = SelectedField::ThreadList;
                            app.set_shown_thread_list(true);

                            thread_list = ThreadList::new();
                            thread_list.set_description(app.selected_board_description());
                            let mut threads: Vec<Thread> = vec![];
                            runtime.block_on(async {
                                let result = client
                                    .get_threads(
                                        app.selected_board().board(),
                                        thread_list.cur_page(),
                                    )
                                    .await;
                                match result {
                                    Ok(data) => threads = data,
                                    Err(err) => eprintln!("{:#?}", err),
                                };

                                app.fill_threads(threads);
                                app.threads.advance_by(1);
                            });
                        }
                        SelectedField::ThreadList => {
                            selected_field = SelectedField::Thread;
                            app.set_shown_thread(true);
                            app.set_shown_board_list(false);

                            let mut thread: Vec<ThreadPost> = vec![];
                            runtime.block_on(async {
                                let result = client
                                    .get_thread(
                                        app.selected_board().board(),
                                        app.selected_thread().posts().first().unwrap().no() as u64,
                                    )
                                    .await;
                                match result {
                                    Ok(data) => thread = data,
                                    Err(err) => eprintln!("{:#?}", err),
                                };

                                app.fill_thread(thread);
                                app.thread.advance_by(1);
                            });
                        }
                        _ => {}
                    };
                }
                _ => {}
            },
            Event::Tick => {
                app.advance_idly();
            }
        }
    }

    Ok(())
}
