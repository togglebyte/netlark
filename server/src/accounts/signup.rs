use std::io::{Write, Read};
use std::io::ErrorKind::WouldBlock;
use std::collections::HashMap;
use std::os::unix::io::AsRawFd;

use netlib::net::tcp::TcpStream;
use netlib::{Interest, Reaction, Reactor, Result, PollReactor};

use crate::connections::Connections;

enum SignUpState {
    Start,
}

pub(super) struct SignUp<T: AsRawFd + Read + Write>(Connections<T>);

impl<T: AsRawFd + Read + Write> SignUp<T> {
    pub(super) fn new() -> Self {
        Self(Connections::new())
    }
}

impl<T: AsRawFd + Read + Write> Reactor for SignUp<T> {
    type Input = PollReactor<T>;
    type Output = Self::Input;

    fn react(&mut self, reaction: Reaction<Self::Input>) -> Reaction<Self::Output> {
        match reaction {
            Reaction::Event(ev) => Reaction::Event(ev),
            Reaction::Value(val) => Reaction::Value(val),
            Reaction::Continue => Reaction::Continue,
        }
    }
}
