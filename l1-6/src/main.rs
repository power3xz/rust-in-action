use std::thread;

fn main() {
    let mut data = 100;

    thread::spawn(|| {
        data = 500;
    });
    thread::spawn(|| {
        data = 1000;
    });

    println!("{}", data);
}

// error[E0373]: closure may outlive the current function, but it borrows `data`, which is owned by the current function
//  --> l1-6/src/main.rs:6:19
//   |
// 6 |     thread::spawn(|| {
//   |                   ^^ may outlive borrowed value `data`
// 7 |         data = 500;
//   |         ---- `data` is borrowed here
//   |
// note: function requires argument type to outlive `'static`
//  --> l1-6/src/main.rs:6:5
//   |
// 6 | /     thread::spawn(|| {
// 7 | |         data = 500;
// 8 | |     });
//   | |______^
// help: to force the closure to take ownership of `data` (and any other referenced variables), use the `move` keyword
//   |
// 6 |     thread::spawn(move || {
//   |                   ^^^^^^^

// error[E0373]: closure may outlive the current function, but it borrows `data`, which is owned by the current function
//  --> l1-6/src/main.rs:6:19
//   |
// 6 |     thread::spawn(|| {
//   |                   ^^ may outlive borrowed value `data`
// 7 |         data = 500;
//   |         ---- `data` is borrowed here
//   |
// note: function requires argument type to outlive `'static`
//  --> l1-6/src/main.rs:6:5
//   |
// 6 | /     thread::spawn(|| {
// 7 | |         data = 500;
// 8 | |     });
//   | |______^
// help: to force the closure to take ownership of `data` (and any other referenced variables), use the `move` keyword
//   |
// 6 |     thread::spawn(move || {
//   |                   ^^^^^^^

// error[E0499]: cannot borrow `data` as mutable more than once at a time
//   --> l1-6/src/main.rs:9:19
//    |
// 6  |       thread::spawn(|| {
//    |       -             -- first mutable borrow occurs here
//    |  _____|
//    | |
// 7  | |         data = 500;
//    | |         ---- first borrow occurs due to use of `data` in closure
// 8  | |     });
//    | |______- argument requires that `data` is borrowed for `'static`
// 9  |       thread::spawn(|| {
//    |                     ^^ second mutable borrow occurs here
// 10 |           data = 1000;
//    |           ---- second borrow occurs due to use of `data` in closure

// error[E0499]: cannot borrow `data` as mutable more than once at a time
//   --> l1-6/src/main.rs:9:19
//    |
// 6  |       thread::spawn(|| {
//    |       -             -- first mutable borrow occurs here
//    |  _____|
//    | |
// 7  | |         data = 500;
//    | |         ---- first borrow occurs due to use of `data` in closure
// 8  | |     });
//    | |______- argument requires that `data` is borrowed for `'static`
// 9  |       thread::spawn(|| {
//    |                     ^^ second mutable borrow occurs here
// 10 |           data = 1000;
//    |           ---- second borrow occurs due to use of `data` in closure

// error[E0373]: closure may outlive the current function, but it borrows `data`, which is owned by the current function
//   --> l1-6/src/main.rs:9:19
//    |
// 9  |     thread::spawn(|| {
//    |                   ^^ may outlive borrowed value `data`
// 10 |         data = 1000;
//    |         ---- `data` is borrowed here
//    |
// note: function requires argument type to outlive `'static`
//   --> l1-6/src/main.rs:9:5
//    |
// 9  | /     thread::spawn(|| {
// 10 | |         data = 1000;
// 11 | |     });
//    | |______^
// help: to force the closure to take ownership of `data` (and any other referenced variables), use the `move` keyword
//    |
// 9  |     thread::spawn(move || {
//    |                   ^^^^^^^

// error[E0373]: closure may outlive the current function, but it borrows `data`, which is owned by the current function
//   --> l1-6/src/main.rs:9:19
//    |
// 9  |     thread::spawn(|| {
//    |                   ^^ may outlive borrowed value `data`
// 10 |         data = 1000;
//    |         ---- `data` is borrowed here
//    |
// note: function requires argument type to outlive `'static`
//   --> l1-6/src/main.rs:9:5
//    |
// 9  | /     thread::spawn(|| {
// 10 | |         data = 1000;
// 11 | |     });
//    | |______^
// help: to force the closure to take ownership of `data` (and any other referenced variables), use the `move` keyword
//    |
// 9  |     thread::spawn(move || {
//    |                   ^^^^^^^

// error[E0502]: cannot borrow `data` as immutable because it is also borrowed as mutable
//   --> l1-6/src/main.rs:13:20
//    |
// 6  |       thread::spawn(|| {
//    |       -             -- mutable borrow occurs here
//    |  _____|
//    | |
// 7  | |         data = 500;
//    | |         ---- first borrow occurs due to use of `data` in closure
// 8  | |     });
//    | |______- argument requires that `data` is borrowed for `'static`
// ...
// 13 |       println!("{}", data);
//    |                      ^^^^ immutable borrow occurs here

// error[E0502]: cannot borrow `data` as immutable because it is also borrowed as mutable
//   --> l1-6/src/main.rs:13:20
//    |
// 6  |       thread::spawn(|| {
//    |       -             -- mutable borrow occurs here
//    |  _____|
//    | |
// 7  | |         data = 500;
//    | |         ---- first borrow occurs due to use of `data` in closure
// 8  | |     });
//    | |______- argument requires that `data` is borrowed for `'static`
// ...
// 13 |       println!("{}", data);
//    |                      ^^^^ immutable borrow occurs here

// error: aborting due to 4 previous errors

// Some errors have detailed explanations: E0373, E0499, E0502.
// For more information about an error, try `rustc --explain E0373`.
// error: aborting due to 4 previous errors

// Some errors have detailed explanations: E0373, E0499, E0502.
// For more information about an error, try `rustc --explain E0373`.
// error: could not compile `l1-6`

// To learn more, run the command again with --verbose.
// error: could not compile `l1-6`

// To learn more, run the command again with --verbose.
