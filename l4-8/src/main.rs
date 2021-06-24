fn use_value(_val: Demo) {}

struct Demo {
    a: i32,
}

fn main() {
    let demo = Demo { a: 123 };
    use_value(demo);

    println!("{}", demo);
}

// Compiling l4-8 v0.1.0 (/Users/devj/project/ria/l4-8)
// error[E0277]: `Demo` doesn't implement `std::fmt::Display`
//   --> l4-8/src/main.rs:11:20
//    |
// 11 |     println!("{}", demo);
//    |                    ^^^^ `Demo` cannot be formatted with the default formatter
//    |
//    = help: the trait `std::fmt::Display` is not implemented for `Demo`
//    = note: in format strings you may be able to use `{:?}` (or {:#?} for pretty-print) instead
//    = note: required by `std::fmt::Display::fmt`
//    = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)

// error: aborting due to previous error

// For more information about this error, try `rustc --explain E0277`.
// error: could not compile `l4-8`

// To learn more, run the command again with --verbose.
