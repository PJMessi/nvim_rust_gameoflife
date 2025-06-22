use crate::internal::{
    app_handler::AppHandler, game_handler::GameHandler, pingpong_handler::PingPongHandler,
};
use async_trait::async_trait;
use nvim_rs::{Handler, Neovim, compat::tokio::Compat};
use rmpv::Value;
use std::sync::Arc;
use tokio::fs::File as TokioFile;
use tokio::sync::Mutex;

#[derive(Clone)]
pub struct NeovimHandler {
    game_handler: Arc<Mutex<GameHandler>>,
    ping_pong_handler: Arc<Mutex<PingPongHandler>>,
}

impl NeovimHandler {
    pub fn new() -> Self {
        NeovimHandler {
            ping_pong_handler: Arc::new(Mutex::new(PingPongHandler::new())),
            game_handler: Arc::new(Mutex::new(GameHandler::new())),
        }
    }
}

#[async_trait]
impl Handler for NeovimHandler {
    type Writer = Compat<TokioFile>;

    async fn handle_request<'a>(
        &'a self,
        name: String,
        _args: Vec<Value>,
        _neovim: Neovim<Compat<TokioFile>>,
    ) -> Result<Value, Value> {
        match name.as_ref() {
            "ping" => {
                let mut handler = self.ping_pong_handler.lock().await;
                handler.handle()
            }
            "gen_grid" => {
                let mut handler = self.game_handler.lock().await;
                handler.handle()
            }
            _ => unimplemented!(),
        }
    }
}
