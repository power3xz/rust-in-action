#[derive(Debug)]
struct CubeSat {
    id: u64,
}

#[derive(Debug)]
enum StatusMessage {
    Ok,
}

fn check_status(sat_id: CubeSat) -> StatusMessage {
    StatusMessage::Ok
}

fn main() {
    let sat_a = CubeSat { id: 0 };
    let sat_b = CubeSat { id: 1 };
    let sat_c = CubeSat { id: 2 };

    let a_status = check_status(sat_a);
    let b_status = check_status(sat_b);
    let c_status = check_status(sat_c);
    println!("a: {:?}, b: {:?}, c: {:?}", a_status, b_status, c_status);

    // waiting...
    let a_status = check_status(sat_a);
    let b_status = check_status(sat_b);
    let c_status = check_status(sat_c);
    println!("a: {:?}, b: {:?}, c: {:?}", a_status, b_status, c_status);
}

// Compiling l4-4 v0.1.0 (/Users/devj/project/ria/l4-4)
// warning: unused variable: `sat_id`
//   --> l4-4/src/main.rs:11:17
//    |
// 11 | fn check_status(sat_id: CubeSat) -> StatusMessage {
//    |                 ^^^^^^ help: if this is intentional, prefix it with an underscore: `_sat_id`
//    |
//    = note: `#[warn(unused_variables)]` on by default

// error[E0382]: use of moved value: `sat_a`
//   --> l4-4/src/main.rs:26:33
//    |
// 16 |     let sat_a = CubeSat { id: 0 };
//    |         ----- move occurs because `sat_a` has type `CubeSat`, which does not implement the `Copy` trait
// ...
// 20 |     let a_status = check_status(sat_a);
//    |                                 ----- value moved here
// ...
// 26 |     let a_status = check_status(sat_a);
//    |                                 ^^^^^ value used here after move

// error[E0382]: use of moved value: `sat_b`
//   --> l4-4/src/main.rs:27:33
//    |
// 17 |     let sat_b = CubeSat { id: 1 };
//    |         ----- move occurs because `sat_b` has type `CubeSat`, which does not implement the `Copy` trait
// ...
// 21 |     let b_status = check_status(sat_b);
//    |                                 ----- value moved here
// ...
// 27 |     let b_status = check_status(sat_b);
//    |                                 ^^^^^ value used here after move

// error[E0382]: use of moved value: `sat_c`
//   --> l4-4/src/main.rs:28:33
//    |
// 18 |     let sat_c = CubeSat { id: 2 };
//    |         ----- move occurs because `sat_c` has type `CubeSat`, which does not implement the `Copy` trait
// ...
// 22 |     let c_status = check_status(sat_c);
//    |                                 ----- value moved here
// ...
// 28 |     let c_status = check_status(sat_c);
//    |                                 ^^^^^ value used here after move

// error: aborting due to 3 previous errors; 1 warning emitted

// For more information about this error, try `rustc --explain E0382`.
// error: could not compile `l4-4`

// To learn more, run the command again with --verbose.
