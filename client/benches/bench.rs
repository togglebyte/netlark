#![feature(test)]
extern crate test;
use florpsylvania::message::Message;
use florpsylvania::pretendserver::codecs;
use test::bench::{black_box, Bencher};

const LOOP_COUNT: usize = 10_000;

#[bench]
fn safe_vec(b: &mut Bencher) {
    b.iter(|| {
        for i in 0..LOOP_COUNT {
            let mut safe_codec = codecs::SafeCodec::new();
            let message = black_box(Message::SignInRequest(
                "Hello".to_string(),
                "hunter".to_string(),
            ));
            safe_codec.encode(&message);
            let message = safe_codec.decode();
        }
    });
}

#[bench]
fn unsafe_alloc_byte_chunk(b: &mut Bencher) {
    b.iter(|| {
        for i in 0..LOOP_COUNT {
            let mut unsafe_codec = codecs::UnsafeCodec::new();
            let message = black_box(Message::SignInRequest(
                "Hello".to_string(),
                "hunter".to_string(),
            ));
            unsafe_codec.encode(&message);
            let message = unsafe_codec.decode();
        }
    });
}
