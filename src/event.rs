use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{mpsc, Arc};
use std::time::Duration;
use std::{io, thread};

use termion::event::Key;
use termion::input::TermRead;

pub(crate) struct Events {
    rx: mpsc::Receiver<Event<Key>>,
    _input_handle: thread::JoinHandle<()>,
    _ignore_exit_key: Arc<AtomicBool>,
    _tick_handle: thread::JoinHandle<()>,
}

pub(crate) enum Event<I> {
    Input(I),
    Tick,
}

impl Events {
    pub(crate) fn new() -> Events {
        Events::with_config(Config::default())
    }

    fn with_config(config: Config) -> Events {
        let (tx, rx) = mpsc::channel();
        let ignore_exit_key = Arc::new(AtomicBool::new(false));
        let input_handle = {
            let tx = tx.clone();
            let ignore_exit_key = ignore_exit_key.clone();
            thread::spawn(move || {
                let stdin = io::stdin();
                for key in stdin.keys().flatten() {
                    if let Err(err) = tx.send(Event::Input(key)) {
                        eprintln!("{}", err);
                        return;
                    }
                    if !ignore_exit_key.load(Ordering::Relaxed) && key == config.exit_key {
                        return;
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

    pub(crate) fn next(&self) -> Result<Event<Key>, mpsc::RecvError> {
        self.rx.recv()
    }

    pub fn _disable_exit_key(&mut self) {
        self._ignore_exit_key.store(true, Ordering::Relaxed);
    }

    pub fn _enable_exit_key(&mut self) {
        self._ignore_exit_key.store(false, Ordering::Relaxed);
    }
}

#[derive(Debug, Clone, Copy)]
struct Config {
    exit_key: Key,
    tick_rate: Duration,
}

impl Default for Config {
    fn default() -> Config {
        Config {
            exit_key: Key::Char('q'),
            tick_rate: Duration::from_millis(250),
        }
    }
}
