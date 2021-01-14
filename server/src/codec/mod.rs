use std::fmt;
use std::io::ErrorKind::WouldBlock;
use std::io::{self, Read, Write};
use std::mem::MaybeUninit;

use netlib::memchr::memchr;

pub mod newline;

const NL: u8 = b'\n';
const BUF_SIZE: usize = 4096;

// -----------------------------------------------------------------------------
//     - Codec -
// -----------------------------------------------------------------------------
pub trait Codec {
    type Item;

    fn decode<T: Read>(&mut self, read_src: &mut T) -> Decode<Self::Item>;
    fn encode<T: Write>(&mut self, write_src: &mut T, data: Self::Item) -> Encode;
}

// -----------------------------------------------------------------------------
//     - Decoder / Decode -
// -----------------------------------------------------------------------------
pub enum Decode<T> {
    Value(T),
    NoValue,
    Failed,
}

impl<T: fmt::Debug> fmt::Debug for Decode<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Decode::Value(val) => write!(f, "Decode::Value({:?})", val),
            Decode::NoValue => write!(f, "Decode::NoValue"),
            Decode::Failed => write!(f, "Decode::Failed"),
        }
    }
}

// -----------------------------------------------------------------------------
//     - Encoder / Encode-
// -----------------------------------------------------------------------------
pub enum Encode {
    Success,
    Fail,
}
