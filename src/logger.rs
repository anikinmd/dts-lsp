use crate::MessageType;
use std::fmt::Display;
use tower_lsp::Client;

#[derive(Clone)]
pub enum Logger {
    Lsp(Client),
}

impl Logger {
    pub async fn log_message<M: Display>(&self, typ: MessageType, message: M) {
        match self {
            Self::Lsp(c) => c.log_message(typ, message).await,
        }
    }
}