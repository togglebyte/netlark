use std::collections::HashMap;
use std::io::ErrorKind::WouldBlock;
use std::io::{Read, Write};
use std::ops::{Deref, DerefMut};
use std::os::unix::io::AsRawFd;

use serde::de::DeserializeOwned;
use serde::Serialize;
use common::frame::Frame;
use netlib::{Event, PollReactor};

// -----------------------------------------------------------------------------
//     - Connections -
// -----------------------------------------------------------------------------
pub struct Connections<T>
where
    T: AsRawFd + Read + Write,
{
    inner: HashMap<u64, (PollReactor<T>, Frame)>,
}

// -----------------------------------------------------------------------------
//     - Deref -
// -----------------------------------------------------------------------------
impl<T> Deref for Connections<T>
where
    T: AsRawFd + Read + Write,
{
    type Target = HashMap<u64, (PollReactor<T>, Frame)>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<T> DerefMut for Connections<T>
where
    T: AsRawFd + Read + Write,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

// -----------------------------------------------------------------------------
//     - Impl -
// -----------------------------------------------------------------------------
impl<T> Connections<T>
where
    T: AsRawFd + Read + Write,
{
    pub fn new() -> Self {
        Self {
            inner: HashMap::new(),
        }
    }

    pub fn update(&mut self, event: &Event) {
        self.inner
            .get_mut(&event.owner)
            .map(|(c, _)| c.update(event));
    }

    pub fn recv<U: DeserializeOwned + std::fmt::Debug>(&mut self, reactor_id: u64) -> Vec<U> {
        let (mut con, mut frame) = match self.remove(&reactor_id) {
            Some((con, frame)) => (con, frame),
            None => return Vec::new(),
        };

        loop {
            match frame.read(&mut con) {
                Ok(0) => return Vec::new(),
                Ok(n) => continue,
                Err(ref e) if e.kind() == WouldBlock => { break }
                Err(e) => return Vec::new(),
            }
        }

        let mut data = Vec::new();

        let mut message = frame.try_msg();
        while let Some(val) = message.take() {
            data.push(val);
        }

        self.insert(reactor_id, (con, frame));
        data
    }

    pub fn send<U: Serialize>(&mut self, reactor_id: u64, message: U) -> Option<()> {
        let (mut con, mut frame) = self.remove(&reactor_id)?;

        let bytes = Frame::frame_message(message);

        match con.write(&bytes) {
            Ok(0) => None,
            Ok(n) => {
                self.insert(reactor_id, (con, frame));
                Some(())
            }
            Err(ref e) if e.kind() == WouldBlock => {
                self.insert(reactor_id, (con, frame));
                None
            }
            Err(e) => None,
        }

    }
}
