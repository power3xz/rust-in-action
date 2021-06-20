use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() {
    let f = File::open("README.md").unwrap();
    let reader = BufReader::new(f);

    for line_ in reader.lines() {
        let line = line_.unwrap();
        println!("{} ({} bytes long)", line, line.len());
    }
}

// ## RUST IN ACTION (17 bytes long)
//  (0 bytes long)
// - exercise for RUST IN ACTION V16 (33 bytes long)
// - https://livebook.manning.com/book/rust-in-action (50 bytes long)
