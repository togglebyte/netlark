use super::*; // TODO: nope

// -----------------------------------------------------------------------------
//     - Newline -
// -----------------------------------------------------------------------------
pub struct Newline {
    pub buf: Vec<u8>,
    bytes_read: usize,
    index: usize,
}

impl Newline {
    pub fn new() -> Self {
        let mut inst = Self {
            buf: Vec::with_capacity(BUF_SIZE),
            bytes_read: 0,
            index: 0,
        };

        unsafe { inst.buf.set_len(BUF_SIZE) };

        inst
    }
}

impl Codec for Newline {
    type Item = Vec<u8>;

    fn decode<T: Read>(&mut self, src: &mut T) -> Decode<Self::Item> {
        // Existing messages
        let buffer = &self.buf[self.index..self.bytes_read];
        match memchr(buffer, NL) {
            Some(pos) => {
                // We have a message
                self.index = pos + 1;
                if self.index >= self.bytes_read {
                    self.index = 0;
                    self.bytes_read = 0;
                }

                let message = buffer[..pos].to_vec();
                return Decode::Value(message);
            }
            None => {
                // If the index is not zero,
                // set index to zero, move all ready bytes down
                // We don't have a message
                self.buf.copy_within(self.index..self.bytes_read, 0);
                self.bytes_read -= self.index;
                self.index = 0;
                // [ ] [ ] [b] [b] we have this
                // [b] [b] [ ] [ ] we want this
            }
        }

        // Fill buffer
        match src.read(&mut self.buf[self.bytes_read..]) {
            Ok(0) => Decode::Failed,
            Ok(n) => {
                self.bytes_read += n;
                self.decode(src)
            }
            Err(ref e) if e.kind() == WouldBlock => Decode::NoValue,
            Err(e) => Decode::Failed,
        }
    }

    fn encode<T: Write>(&mut self, src: &mut T, mut data: Self::Item) -> Encode {
        match src.write(&data).is_ok() {
            true => Encode::Success,
            false => Encode::Fail,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::io::{BufReader, Read, Result};

    struct SillyReader;

    impl Read for SillyReader {
        fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
            let src = "Hello world\n".as_bytes();
            let len = buf.len().min(src.len());
            buf[..len].copy_from_slice(&src[..len]);
            Ok(len)
        }
    }

    #[test]
    fn decode_multiple() {
        let mut sr = SillyReader;
        let mut nl = Newline::new();
        let len = nl.buf.len();
        for _ in 0..10 {
            let msg = nl.decode(&mut sr);
        }
    }
}

