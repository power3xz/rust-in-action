fn main() {
    let mut letters = vec!["a", "b", "b"];

    for letter in letters {
        println!("{}", letter);
        letters.push(letter.clone());
    }
}

// error[E0382]: borrow of moved value: `letters`
//    --> l1-8/src/main.rs:6:9
//     |
// 2   |     let mut letters = vec!["a", "b", "b"];
//     |         ----------- move occurs because `letters` has type `Vec<&str>`, which does not implement the `Copy` trait
// 3   |
// 4   |     for letter in letters {
//     |                   -------
//     |                   |
//     |                   `letters` moved due to this implicit call to `.into_iter()`
//     |                   help: consider borrowing to avoid moving into the for loop: `&letters`
// 5   |         println!("{}", letter);
// 6   |         letters.push(letter.clone());
//     |         ^^^^^^^ value borrowed here after move
//     |
// note: this function takes ownership of the receiver `self`, which moves `letters`
//    --> /Users/devj/.rustup/toolchains/stable-x86_64-apple-darwin/lib/rustlib/src/rust/library/core/src/iter/traits/collect.rs:232:18
//     |
// 232 |     fn into_iter(self) -> Self::IntoIter;
//     |                  ^^^^

// error[E0382]: borrow of moved value: `letters`
//    --> l1-8/src/main.rs:6:9
//     |
// 2   |     let mut letters = vec!["a", "b", "b"];
//     |         ----------- move occurs because `letters` has type `Vec<&str>`, which does not implement the `Copy` trait
// 3   |
// 4   |     for letter in letters {
//     |                   -------
//     |                   |
//     |                   `letters` moved due to this implicit call to `.into_iter()`
//     |                   help: consider borrowing to avoid moving into the for loop: `&letters`
// 5   |         println!("{}", letter);
// 6   |         letters.push(letter.clone());
//     |         ^^^^^^^ value borrowed here after move
//     |
// note: this function takes ownership of the receiver `self`, which moves `letters`
//    --> /Users/devj/.rustup/toolchains/stable-x86_64-apple-darwin/lib/rustlib/src/rust/library/core/src/iter/traits/collect.rs:232:18
//     |
// 232 |     fn into_iter(self) -> Self::IntoIter;
//     |                  ^^^^

// error: aborting due to previous error

// error: aborting due to previous error

// For more information about this error, try `rustc --explain E0382`.
// For more information about this error, try `rustc --explain E0382`.
// error: error: could not compile `l1-8`

// To learn more, run the command again with --verbose.
// could not compile `l1-8`

// To learn more, run the command again with --verbose.
