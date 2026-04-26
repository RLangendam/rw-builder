use std::io::{Read, BufRead, BufReader};
fn main() {
    let r: std::io::Empty = std::io::empty();
    let d = zstd::stream::read::Decoder::new(r).unwrap();
    let _: () = d;
}
