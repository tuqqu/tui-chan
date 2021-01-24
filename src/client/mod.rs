use std::error::Error;

use reqwest::Client;


use crate::{
    client::api::ChanApi,
    client::response::{BoardListResponse, ThreadListResponse, ThreadResponse},
    model::{Board, Thread, ThreadPost},
};

pub mod api;
mod response;

pub struct ChanClient {
    client: Client,
    api: &'static dyn ChanApi,
}

impl ChanClient {
    pub fn new(client: Client, api: &'static dyn ChanApi) -> ChanClient {
        ChanClient { api, client }
    }

    pub async fn get_boards(&self) -> Result<Vec<Board>, Box<dyn Error>> {
        let boards_response: BoardListResponse = self
            .client
            .get(&self.api.boards())
            .send()
            .await?
            .json::<BoardListResponse>()
            .await?;

        Ok(boards_response.boards)
    }

    pub async fn get_threads(
        &self,
        board: &str,
        page: u8,
    ) -> Result<Vec<Thread>, Box<dyn Error>> {
        let threads_response: ThreadListResponse = self
            .client
            .get(&self.api.threads(board, page))
            .send()
            .await?
            .json::<ThreadListResponse>()
            .await?;

        Ok(threads_response.threads)
    }

    pub async fn get_thread(
        &self,
        board: &str,
        no: u64,
    ) -> Result<Vec<ThreadPost>, Box<dyn Error>> {
        let thread_response: ThreadResponse = self
            .client
            .get(&self.api.thread(board, no))
            .send()
            .await?
            .json::<ThreadResponse>()
            .await?;

        Ok(thread_response.posts)
    }
}
