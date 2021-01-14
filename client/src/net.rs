use std::io::ErrorKind::WouldBlock;
use std::io::{self, Read, Write};
use std::net::TcpStream;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

use common::frame::Frame;
use common::models::Message;
use common::{Rx, Tx};

use crate::loggy;

pub fn connect(client_tx: Tx) -> Tx {
    let (server_tx, server_rx) = mpsc::channel::<Message>();

    let mut connection = TcpStream::connect("127.0.0.1:9000").unwrap();
    connection.set_nonblocking(true);

    let handle = thread::spawn(move || {
        let mut frame = Frame::empty();

        loop {
            // -----------------------------------------------------------------------------
            //     - Receiving messages -
            // -----------------------------------------------------------------------------
            match frame.read(&mut connection) {
                Ok(0) => {
                    eprintln!("Connection closed");
                    return;
                }
                Ok(n) => {
                    if let Some(message) = frame.try_msg::<Message>() {
                        client_tx.send(message);
                    }
                }
                Err(ref e) if e.kind() == WouldBlock => {},
                Err(e) => {
                    eprintln!("connection error: {:?}", e);
                    return;
                }
            }

            // -----------------------------------------------------------------------------
            //     - Sending messages -
            // -----------------------------------------------------------------------------
            match server_rx.try_recv() {
                Ok(mut message) => {
                    loggy!(message);
                    let bytes = Frame::frame_message(message);
                    let _ = connection.write_all(&bytes);
                }
                Err(_) => {}
            }

            thread::sleep(Duration::from_millis(20));
        }
    });

    server_tx
}
