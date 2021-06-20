use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() {
    let f = File::open("README.md").unwrap();
    let mut reader = BufReader::new(f);

    let mut line = String::new();

    loop {
        let len = reader.read_line(&mut line).unwrap();
        if len == 0 {
            break;
        }
        println!("{} ({} bytes long)", line, len);
        line.truncate(0);
    }
}

// ## RUST IN ACTION
//  (18 bytes long)

//  (1 bytes long)
// - exercise for RUST IN ACTION V16
//  (34 bytes long)
// - https://livebook.manning.com/book/rust-in-action
//  (51 bytes long)
