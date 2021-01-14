use std::sync::mpsc::{self, Sender, Receiver};

pub mod models;
pub mod frame;

use models::Message;

pub type Rx = Receiver<Message>;
pub type Tx = Sender<Message>;

pub fn channel() -> (Tx, Rx) {
    mpsc::channel()
}
