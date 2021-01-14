#![allow(warnings)]
use std::thread;

use netlib::net::tcp::{TcpListener, TcpStream};
use netlib::queue::Worker;
use netlib::{Reactor, Result, System, Interest};

mod accounts;
// mod codec;
mod connections;
mod datastore;

const THREAD_COUNT: usize = 8;

pub fn run() -> Result<()> {
    System::builder().finish();
    let listener = TcpListener::bind("127.0.0.1:9000")?
        .filter_map(Result::ok)
        .map(|(s, _)| {
            s.set_nonblocking(true);
            s
        });

    let mut queue = Worker::new()?;

    // Thread handles
    let mut handles = vec![];

    for _ in 0..THREAD_COUNT {
        let mut stealer = queue.dequeue()?;
        let handle = thread::spawn(|| -> Result<()> {
            System::builder().finish();
            stealer.arm();

            let server = stealer
                .filter_map(Result::ok)
                .map(|std_stream| TcpStream::new(std_stream, Interest::ReadWrite))
                .filter_map(Result::ok).chain(accounts::accounts());

            System::start(server);
            Ok(())
        });
        handles.push(handle);
    }

    System::start(listener.chain(queue));

    handles.into_iter().for_each(|h| {
        h.join();
    });

    Ok(())
}
