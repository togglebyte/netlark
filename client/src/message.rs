use std::sync::mpsc::{self, Sender, Receiver};

use tinybit::{WorldPos, Pixel};
use serde::{Deserialize, Serialize};

pub type Rx = Receiver<Message>;
pub type Tx = Sender<Message>;

pub fn client_channel() -> (Tx, Rx) {
    mpsc::channel()
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SaveState {
    pub tilemap: Vec<Pixel>,
    pub player_pos: WorldPos,
    pub player_id: usize,
    pub inventory: Vec<()>,
}


#[derive(Debug, Serialize, Deserialize)]
pub enum Message {
    SignInRequest(String, String),
    SignInResponse(SaveState),
}
