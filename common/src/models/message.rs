use serde::{Deserialize, Serialize};

use super::GameState;

#[derive(Debug, Serialize, Deserialize)]
pub enum Auth {
    Failed,
    Success(GameState),
    SignIn(String, String),
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Message {
    Auth(Auth),
}
