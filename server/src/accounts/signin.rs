use std::collections::HashMap;
use std::io::ErrorKind::WouldBlock;
use std::io::{Read, Write};
use std::os::unix::io::AsRawFd;

use common::frame::Frame;
use common::models::{Auth, Message};
use netlib::net::tcp::TcpStream;
use netlib::{Interest, PollReactor, Reaction, Reactor, Result};

use crate::connections::Connections;
use crate::datastore;

fn authenticate(username: &str, password: &str) -> bool {
    username == "f" && password == "t"
}

// -----------------------------------------------------------------------------
//     - Sign In reactor -
// -----------------------------------------------------------------------------
pub struct SignIn<T: AsRawFd + Read + Write>(Connections<T>);

impl<T: AsRawFd + Read + Write> SignIn<T> {
    pub(super) fn new() -> Self {
        Self(Connections::new())
    }
}

impl<T: AsRawFd + Read + Write> Reactor for SignIn<T> {
    type Input = PollReactor<T>;
    type Output = Self::Input;

    fn react(&mut self, reaction: Reaction<Self::Input>) -> Reaction<Self::Output> {
        match reaction {
            Reaction::Event(ev) if self.0.contains_key(&ev.owner) => {
                let messages = self.0.recv(ev.owner);
                match messages.first() {
                    Some(Message::Auth(Auth::SignIn(u, p))) => {
                        if !authenticate(&u, &p) {
                            eprintln!("{:?}", "failed auth");
                            self.0.send(ev.owner, Message::Auth(Auth::Failed));
                            return Reaction::Continue;
                        }

                        // TODO omg no (we are making a fake game state, let's not)
                        let gamestate = datastore::get_game_state(&u).unwrap();

                        self.0
                            .send(ev.owner, Message::Auth(Auth::Success(gamestate)));
                        eprintln!("{:?}", "Sent successful state");
                    }
                    _ => {
                        eprintln!("message: {:?}", messages);
                    }
                }

                Reaction::Continue
            }

            // -----------------------------------------------------------------------------
            //     - Incoming connections -
            // -----------------------------------------------------------------------------
            Reaction::Value(val) => {
                self.0.insert(val.id, (val, Frame::empty()));
                Reaction::Continue
            }

            Reaction::Event(ev) => Reaction::Event(ev),
            Reaction::Continue => Reaction::Continue,
        }
    }
}
