#[derive(PartialEq)]
struct Hostname(String);

fn main() {
    let ordinary_string = String::from("localhost");
    let host = Hostname(ordinary_string.clone());
    if host == ordinary_string {
        println!("huh?");
    };
}

// error[E0308]: mismatched types
//  --> l3-7/src/main.rs:7:16
//   |
// 7 |     if host == ordinary_string {
//   |                ^^^^^^^^^^^^^^^ expected struct `Hostname`, found struct `String`

// error: aborting due to previous error

// For more information about this error, try `rustc --explain E0308`.
// error: could not compile `l3-7`

// To learn more, run the command again with --verbose.
