use common::models::{Auth, Message};
use common::frame::Frame;
use std::fs::File;
use std::io::{self, Read};

fn main() -> io::Result<()> {
    let mut file = File::open("/tmp/payload")?;
    let mut frame = Frame::empty();

    loop {
        let n = frame.read(&mut file)?;
        let msg = frame.try_msg::<Message>();
        eprintln!("{} | {}", n, msg.is_some());
        if n == 0 {
            break
        }
    }

    Ok(())
}
