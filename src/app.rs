use tui::widgets::ListState;

use crate::client::api::ContentUrlProvider;
use crate::format::format_html;
use crate::keybinds::{display_key, Keybinds};
use crate::model::{Board, Thread, ThreadPost};
use crate::style::SelectedField;

pub(crate) struct App {
    pub(crate) boards: ItemLIst<Board>,
    pub(crate) threads: ItemLIst<Thread>,
    pub(crate) thread: ItemLIst<ThreadPost>,
    shown_state: ShownState,
    help_bar: HelpBar,
}

/// Format 2D array as table, with aligned columns
fn format_table(data: &[&[&str]]) -> String {
    // Find the maximum length of each column
    let mut max_lengths = vec![0; data[0].len()];
    for row in data {
        for (i, &cell) in row.iter().enumerate() {
            max_lengths[i] = max_lengths[i].max(cell.len());
        }
    }
    // Compile table
    let mut rows = Vec::new();
    for row in data {
        let mut cells = Vec::new();
        for (i, &cell) in row.iter().enumerate() {
            cells.push(format!("{:<width$} ", cell, width = max_lengths[i] + 3));
        }
        rows.push(cells.join(""));
    }
    rows.join("\n")
}

impl App {
    pub(crate) fn new(
        boards: Vec<Board>,
        threads: Vec<Thread>,
        thread: Vec<ThreadPost>,
        keybinds: &Keybinds,
    ) -> Self {
        /// Get keybinds as strings
        macro_rules! get_keys {
            ( $($name:ident),* $(,)? ) => {
                $( let $name = display_key(&keybinds.$name);)*
            }
        }
        get_keys![
            up,
            down,
            left,
            right,
            quick_up,
            quick_down,
            quick_left,
            quick_right,
            page_next,
            page_previous,
            copy_thread,
            open_thread,
            copy_media,
            open_media,
            fullscreen,
            reload,
            help,
            quit,
        ];

        // Create table of keybinds
        let table: &[&[&str]] = &[
            &[
                "move around:",
                &format!("{up}, {down}, {left}, {right}"),
                "toggle help bar:",
                &help,
            ],
            &[
                "move quickly:",
                &format!("{quick_up}, {quick_down}, {quick_left}, {quick_right}"),
                "copy thread/post url:",
                &copy_thread,
            ],
            &[
                "toggle fullscreen:",
                &fullscreen,
                "copy media url:",
                &copy_media,
            ],
            &[
                "next page:",
                &page_next,
                "open thread/post in browser",
                &open_thread,
            ],
            &["previous page:", &page_previous, "reload page:", &reload],
            &["quit:", &quit, "open media url in browser:", &open_media],
        ];

        let text = format!(
            r##"
                {table}
                Controls can be changed in ~/.config/tui-chan/keybinds.conf
                Note: to enter the board/thread use "{right}"
            "##,
            table = format_table(table)
        );

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
                title: format!("Help (\"{help}\" to toggle)"),
                text,
            },
        }
    }

    pub(crate) fn fill_threads(&mut self, threads: Vec<Thread>) {
        self.threads = ItemLIst::new(threads);
    }

    pub(crate) fn fill_thread(&mut self, thread: Vec<ThreadPost>) {
        self.thread = ItemLIst::new(thread);
    }

    pub(crate) fn advance_idly(&self) {}

    pub(crate) fn advance(&mut self, selected_field: &SelectedField, steps: isize) {
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

    pub(crate) fn calc_screen_share(&self) -> ScreenShare {
        match (
            self.shown_state.board_list,
            self.shown_state.thread_list,
            self.shown_state.thread,
        ) {
            (true, false, false) => ScreenShare::new(100, 0, 0),
            (true, true, false) => ScreenShare::new(12, 88, 0),
            (true, true, true) => ScreenShare::new(12, 88, 50), // check
            (false, true, true) => ScreenShare::new(12, 34, 54),
            (false, false, true) => ScreenShare::new(0, 0, 100),
            (false, true, false) => ScreenShare::new(0, 100, 0),
            _ => ScreenShare::new(100, 0, 0),
        }
    }

    pub(crate) fn selected_board(&self) -> &Board {
        &self.boards.items[self.boards.state.selected().unwrap_or(0)]
    }

    pub(crate) fn selected_thread(&self) -> &Thread {
        &self.threads.items[self.threads.state.selected().unwrap_or(0)]
    }

    pub(crate) fn selected_thread_description(&self) -> String {
        if let Some(post_i) = self.threads.state.selected() {
            let thread = &self.threads.items[post_i];
            let post = thread.posts().first().unwrap();
            let title = format_html(post.sub());
            let title = if title.is_empty() {
                "".to_string()
            } else {
                format!("\"{}\" ", title)
            };

            format!("{} {}replies: {} ", post.no(), title, post.replies())
        } else {
            "".to_string()
        }
    }

    pub(crate) fn selected_post(&self) -> &ThreadPost {
        &self.thread.items[self.thread.state.selected().unwrap()]
    }

    pub(crate) fn set_shown_board_list(&mut self, shown: bool) {
        self.shown_state.board_list = shown;
    }

    pub(crate) fn set_shown_thread_list(&mut self, shown: bool) {
        self.shown_state.thread_list = shown;
    }

    pub(crate) fn set_shown_thread(&mut self, shown: bool) {
        self.shown_state.thread = shown;
    }

    pub(crate) fn toggle_shown_board_list(&mut self) {
        self.shown_state.board_list ^= true;
    }

    pub(crate) fn toggle_shown_thread_list(&mut self) {
        self.shown_state.thread_list ^= true;
    }

    #[allow(dead_code)]
    pub(crate) fn toggle_shown_thread(&mut self) {
        self.shown_state.thread ^= true;
    }

    #[allow(dead_code)]
    pub(crate) fn shown_board_list(&mut self) -> bool {
        self.shown_state.board_list
    }

    pub(crate) fn shown_thread_list(&mut self) -> bool {
        self.shown_state.thread_list
    }

    pub(crate) fn shown_thread(&mut self) -> bool {
        self.shown_state.thread
    }

    pub(crate) fn help_bar(&self) -> &HelpBar {
        &self.help_bar
    }

    pub(crate) fn help_bar_mut(&mut self) -> &mut HelpBar {
        &mut self.help_bar
    }

    pub(crate) fn url_boards(&self, url_provider: &dyn ContentUrlProvider) -> String {
        url_provider.url_board(self.selected_board().board())
    }

    pub(crate) fn url_threads(&self, url_provider: &dyn ContentUrlProvider) -> String {
        url_provider.url_thread(
            self.selected_board().board(),
            self.selected_thread().posts().first().unwrap().no() as u64,
        )
    }

    pub(crate) fn url_thread(&self, url_provider: &dyn ContentUrlProvider) -> String {
        url_provider.url_thread_post(
            self.selected_board().board(),
            self.selected_thread().posts().first().unwrap().no() as u64,
            self.selected_post().no() as u64,
        )
    }

    pub(crate) fn media_url_threads(
        &self,
        url_provider: &dyn ContentUrlProvider,
    ) -> Option<String> {
        let post = self.selected_thread().posts().first().unwrap();
        self.media_url(post, url_provider)
    }

    pub(crate) fn media_url_thread(&self, url_provider: &dyn ContentUrlProvider) -> Option<String> {
        let post = self.selected_post();
        self.media_url(post, url_provider)
    }

    fn media_url(
        &self,
        post: &ThreadPost,
        url_provider: &dyn ContentUrlProvider,
    ) -> Option<String> {
        if post.tim().is_none() || post.ext().is_none() {
            return None;
        }

        let url = url_provider.url_file(
            self.selected_board().board(),
            format!(
                "{}{}",
                post.tim().as_ref().unwrap(),
                post.ext().as_ref().unwrap()
            ),
        );

        Some(url)
    }
}

pub(crate) struct ScreenShare {
    board_list: u16,
    thread_list: u16,
    thread: u16,
}

impl ScreenShare {
    fn new(board_list: u16, thread_list: u16, thread: u16) -> ScreenShare {
        ScreenShare {
            board_list,
            thread_list,
            thread,
        }
    }

    pub(crate) fn board_list(&self) -> u16 {
        self.board_list
    }

    pub(crate) fn thread_list(&self) -> u16 {
        self.thread_list
    }

    pub(crate) fn thread(&self) -> u16 {
        self.thread
    }
}

struct ShownState {
    board_list: bool,
    thread_list: bool,
    thread: bool,
}

pub(crate) struct ItemLIst<T> {
    pub(crate) state: ListState,
    pub(crate) items: Vec<T>,
}

pub(crate) struct HelpBar {
    shown: bool,
    title: String,
    text: String,
}

impl HelpBar {
    pub(crate) fn shown(&self) -> bool {
        self.shown
    }

    pub(crate) fn toggle_shown(&mut self) {
        self.shown ^= true;
    }

    pub(crate) fn title(&self) -> &String {
        &self.title
    }

    pub(crate) fn text(&self) -> &String {
        &self.text
    }
}

impl<T> ItemLIst<T> {
    pub(crate) fn new(items: Vec<T>) -> ItemLIst<T> {
        ItemLIst {
            state: ListState::default(),
            items,
        }
    }

    pub(crate) fn advance_by(&mut self, steps: isize) {
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

    pub(crate) fn _unselect(&mut self) {
        self.state.select(None);
    }
}
