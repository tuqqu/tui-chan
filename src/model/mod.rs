use serde::{Deserialize, Serialize};


#[derive(Debug, Serialize, Deserialize)]
pub struct Board {
    pub board: String,
    pub title: String,
    pub meta_description: String,
    pub per_page: isize,
    pub pages: isize,
    pub bump_limit: isize,
}

pub struct ThreadList {
    page: u8,
}

impl ThreadList {
    pub fn new() -> ThreadList {
        ThreadList { page: 1 }
    }

    pub fn next_page(&mut self) -> u8 {
        let page = self.page;
        self.page += 1;
        page
    }

    pub fn prev_page(&mut self) -> u8 {
        let page = self.page;
        self.page -= 1;
        page
    }

    pub fn cur_page(&self) -> u8 {
        self.page
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Thread {
    pub posts: Vec<ThreadPost>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ThreadPost {
    #[serde(default)]
    pub no: usize,
    #[serde(default)]
    pub now: String,
    #[serde(default)]
    pub time: u64,
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub com: String,
    #[serde(default)]
    pub sub: String,
    #[serde(default)]
    pub sticky: u8,
    #[serde(default)]
    pub closed: u8,
    #[serde(default)]
    pub replies: u32,
    #[serde(default)]
    pub ext: Option<String>,
    #[serde(default)]
    pub filename: Option<String>,
    #[serde(default)]
    pub tim: Option<u64>,
}
