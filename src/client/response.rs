use serde::{Deserialize, Serialize};

use crate::model::{Board, Thread, ThreadPost};

#[derive(Debug, Serialize, Deserialize)]
pub(super) struct BoardListResponse {
    pub(super) boards: Vec<Board>,
}

#[derive(Debug, Serialize, Deserialize)]
pub(super) struct ThreadListResponse {
    pub(super) threads: Vec<Thread>,
}

#[derive(Debug, Serialize, Deserialize)]
pub(super) struct ThreadResponse {
    pub(super) posts: Vec<ThreadPost>,
}
