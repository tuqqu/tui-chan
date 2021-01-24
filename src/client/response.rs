use serde::{Deserialize, Serialize};


use crate::model::{Board, Thread, ThreadPost};

#[derive(Debug, Serialize, Deserialize)]
pub struct BoardListResponse {
    pub boards: Vec<Board>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ThreadListResponse {
    pub threads: Vec<Thread>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ThreadResponse {
    pub posts: Vec<ThreadPost>,
}
