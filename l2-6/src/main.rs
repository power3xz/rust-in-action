use std::convert::TryInto;

fn main() {
    let a: i32 = 10;
    let b: u16 = 100;

    if a < b.try_into().unwrap() {
        println!("Ten is less than on hundred.");
    }
}

// Ten is less than on hundred.
