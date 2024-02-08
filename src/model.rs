use crate::format::format_html;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct Board {
    board: String,
    title: String,
    meta_description: String,
    per_page: isize,
    pages: isize,
    bump_limit: isize,
}

impl Board {
    pub(crate) fn board(&self) -> &str {
        &self.board
    }

    pub(crate) fn title(&self) -> &str {
        &self.title
    }

    pub(crate) fn meta_description(&self) -> &str {
        &self.meta_description
    }

    #[allow(dead_code)]
    pub(crate) fn per_page(&self) -> isize {
        self.per_page
    }

    #[allow(dead_code)]
    pub(crate) fn pages(&self) -> isize {
        self.pages
    }

    #[allow(dead_code)]
    pub(crate) fn bump_limit(&self) -> isize {
        self.bump_limit
    }
}

pub struct ThreadList {
    page: u8,
    description: String,
}

impl ThreadList {
    const DEFAULT: u8 = 1;

    pub(crate) fn new() -> Self {
        Self {
            page: Self::DEFAULT,
            description: "".to_string(),
        }
    }

    pub(crate) fn next_page(&mut self, board: &Board) -> u8 {
        if board.pages as u8 == self.page {
            self.page = Self::DEFAULT;
        } else {
            self.page += 1;
        }

        self.page
    }

    pub(crate) fn prev_page(&mut self, board: &Board) -> u8 {
        if Self::DEFAULT == self.page {
            self.page = board.pages as u8;
        } else {
            self.page -= 1;
        }

        self.page
    }

    pub(crate) fn cur_page(&self) -> u8 {
        self.page
    }

    pub(crate) fn set_description(&mut self, desc: &str) {
        self.description = format_html(desc);
    }

    pub(crate) fn description(&self) -> &str {
        &self.description
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Thread {
    posts: Vec<ThreadPost>,
}

impl Thread {
    pub(crate) fn posts(&self) -> &[ThreadPost] {
        &self.posts
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ThreadPost {
    #[serde(default)]
    no: usize,
    #[serde(default)]
    now: String,
    #[serde(default)]
    time: u64,
    #[serde(default)]
    id: String,
    #[serde(default)]
    name: String,
    #[serde(default)]
    com: String,
    #[serde(default)]
    sub: String,
    #[serde(default)]
    sticky: u8,
    #[serde(default)]
    closed: u8,
    #[serde(default)]
    replies: u32,
    #[serde(default)]
    ext: Option<String>,
    #[serde(default)]
    filename: Option<String>,
    #[serde(default)]
    tim: Option<u64>,
}

impl ThreadPost {
    pub(crate) fn no(&self) -> usize {
        self.no
    }

    pub(crate) fn time(&self) -> u64 {
        self.time
    }

    pub(crate) fn name(&self) -> &str {
        &self.name
    }

    pub(crate) fn com(&self) -> &str {
        &self.com
    }

    pub(crate) fn sub(&self) -> &str {
        &self.sub
    }

    pub(crate) fn sticky(&self) -> u8 {
        self.sticky
    }

    pub(crate) fn closed(&self) -> u8 {
        self.closed
    }

    pub(crate) fn replies(&self) -> u32 {
        self.replies
    }

    pub(crate) fn ext(&self) -> &Option<String> {
        &self.ext
    }

    pub(crate) fn filename(&self) -> &Option<String> {
        &self.filename
    }

    pub(crate) fn tim(&self) -> Option<u64> {
        self.tim
    }

    #[allow(dead_code)]
    pub(crate) fn now(&self) -> &str {
        &self.now
    }

    #[allow(dead_code)]
    pub(crate) fn id(&self) -> &str {
        &self.id
    }
}
