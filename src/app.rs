use std::{
    io,
    sync::{
        atomic::{AtomicBool, Ordering},
        mpsc, Arc,
    },
    thread,
    time::Duration,
};

use termion::{event::Key, input::TermRead};
use tui::widgets::ListState;

use crate::model::{Board, Thread, ThreadPost};
use crate::view::SelectedField;

pub struct App {
    pub boards: ItemLIst<Board>,
    pub threads: ItemLIst<Thread>,
    pub thread: ItemLIst<ThreadPost>,
    pub shown_state: ShownState,
    pub help_bar: HelpBar,
}

impl App {
    pub fn new(boards: Vec<Board>, threads: Vec<Thread>, thread: Vec<ThreadPost>) -> Self {
        Self {
            boards: ItemLIst::new(boards),
            threads: ItemLIst::new(threads),
            thread: ItemLIst::new(thread),
            shown_state: ShownState {
                board_list: false,
                thread_list: false,
                thread: false,
            },
            help_bar: HelpBar {
                shown: false,
                title: "Help (\"h\" to toggle)",
                text: r##"
                move around:               w,a,s,d or arrows     toggle help bar:            h
                move quickly:              CTRL + w,a,s,d        paginate threads:           p
                toggle fullscreen:         z                     quit:                       q
                copy selected item url:    c
                copy item media url:       CTRL + c

                Note: to enter the board/thread use "d" or "->" key.
                "##,
            },
        }
    }

    pub fn fill_threads(&mut self, threads: Vec<Thread>) {
        self.threads = ItemLIst::new(threads);
    }

    pub fn fill_thread(&mut self, thread: Vec<ThreadPost>) {
        self.thread = ItemLIst::new(thread);
    }

    pub fn advance_idly(&self) {}

    pub fn advance(&mut self, selected_field: &SelectedField, steps: isize) {
        match selected_field {
            SelectedField::BoardList => {
                self.boards.advance_by(steps);
            }
            SelectedField::ThreadList => {
                self.threads.advance_by(steps);
            }
            SelectedField::Thread => {
                self.thread.advance_by(steps);
            }
        };
    }

    pub fn calc_screen_share(&self) -> ScreenShare {
        match (
            self.shown_state.board_list,
            self.shown_state.thread_list,
            self.shown_state.thread,
        ) {
            (true, false, false) => ScreenShare::new(100, 0, 0),
            (true, true, false) => ScreenShare::new(12, 88, 0),
            (true, true, true) => ScreenShare::new(12, 88, 50), // todo? why it is not the case
            (false, true, true) => ScreenShare::new(12, 34, 54),
            (false, false, true) => ScreenShare::new(0, 0, 100),
            (false, true, false) => ScreenShare::new(0, 100, 0),
            _ => ScreenShare::new(100, 0, 0),
        }
    }
}

pub struct ScreenShare {
    pub board_list: u16,
    pub thread_list: u16,
    pub thread: u16,
}

impl ScreenShare {
    fn new(board_list: u16, thread_list: u16, thread: u16) -> ScreenShare {
        ScreenShare {
            board_list,
            thread_list,
            thread,
        }
    }
}

pub struct ShownState {
    pub board_list: bool,
    pub thread_list: bool,
    pub thread: bool,
}

pub struct ItemLIst<T> {
    pub state: ListState,
    pub items: Vec<T>,
}

pub struct HelpBar {
    pub shown: bool,
    pub title: &'static str,
    pub text: &'static str,
}

impl<T> ItemLIst<T> {
    pub fn new(items: Vec<T>) -> ItemLIst<T> {
        ItemLIst {
            state: ListState::default(),
            items,
        }
    }

    pub fn advance_by(&mut self, steps: isize) {
        let selected = match self.state.selected() {
            Some(selected) => {
                if selected as isize >= self.items.len() as isize - steps {
                    0_isize
                } else if selected == 0 && steps < 0 {
                    self.items.len() as isize - 1
                } else {
                    selected as isize + steps
                }
            }
            None => 0,
        };

        self.state.select(Some(selected as usize));
    }

    pub fn _unselect(&mut self) {
        self.state.select(None);
    }
}

pub struct Events {
    rx: mpsc::Receiver<Event<Key>>,
    _input_handle: thread::JoinHandle<()>,
    _ignore_exit_key: Arc<AtomicBool>,
    _tick_handle: thread::JoinHandle<()>,
}

pub enum Event<I> {
    Input(I),
    Tick,
}

#[derive(Debug, Clone, Copy)]
pub struct Config {
    pub exit_key: Key,
    pub tick_rate: Duration,
}

impl Default for Config {
    fn default() -> Config {
        Config {
            exit_key: Key::Char('q'),
            tick_rate: Duration::from_millis(250),
        }
    }
}

impl Events {
    pub fn new() -> Events {
        Events::with_config(Config::default())
    }

    pub fn with_config(config: Config) -> Events {
        let (tx, rx) = mpsc::channel();
        let ignore_exit_key = Arc::new(AtomicBool::new(false));
        let input_handle = {
            let tx = tx.clone();
            let ignore_exit_key = ignore_exit_key.clone();
            thread::spawn(move || {
                let stdin = io::stdin();
                for evt in stdin.keys() {
                    if let Ok(key) = evt {
                        if let Err(err) = tx.send(Event::Input(key)) {
                            eprintln!("{}", err);
                            return;
                        }
                        if !ignore_exit_key.load(Ordering::Relaxed) && key == config.exit_key {
                            return;
                        }
                    }
                }
            })
        };

        let tick_handle = {
            thread::spawn(move || loop {
                if tx.send(Event::Tick).is_err() {
                    break;
                }
                thread::sleep(config.tick_rate);
            })
        };

        Events {
            rx,
            _ignore_exit_key: ignore_exit_key,
            _input_handle: input_handle,
            _tick_handle: tick_handle,
        }
    }

    pub fn next(&self) -> Result<Event<Key>, mpsc::RecvError> {
        self.rx.recv()
    }

    pub fn _disable_exit_key(&mut self) {
        self._ignore_exit_key.store(true, Ordering::Relaxed);
    }

    pub fn _enable_exit_key(&mut self) {
        self._ignore_exit_key.store(false, Ordering::Relaxed);
    }
}
