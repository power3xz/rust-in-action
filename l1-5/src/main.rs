#[derive(Debug)]
enum Cereal {
    Barley,
    Millet,
    Rice,
    Rye,
    Spelt,
    Wheat,
}

fn main() {
    let mut grains: Vec<Cereal> = vec![];

    grains.push(Cereal::Rye);
    drop(grains);

    println!("{:?}", grains)
}
// error[E0382]: borrow of moved value: `grains`
//   --> l1-5/src/main.rs:17:22
//    |
// 12 |     let mut grains: Vec<Cereal> = vec![];
//    |         ---------- move occurs because `grains` has type `Vec<Cereal>`, which does not implement the `Copy` trait
// ...
// 15 |     drop(grains);
//    |          ------ value moved here
// 16 |
// 17 |     println!("{:?}", grains)
//    |                      ^^^^^^ value borrowed here after move

// error[E0382]: borrow of moved value: `grains`
//   --> l1-5/src/main.rs:17:22
//    |
// 12 |     let mut grains: Vec<Cereal> = vec![];
//    |         ---------- move occurs because `grains` has type `Vec<Cereal>`, which does not implement the `Copy` trait
// ...
// 15 |     drop(grains);
//    |          ------ value moved here
// 16 |
// 17 |     println!("{:?}", grains)
//    |                      ^^^^^^ value borrowed here after move

// error: aborting due to previous error

// For more information about this error, try `rustc --explain E0382`.
// error: aborting due to previous error

// For more information about this error, try `rustc --explain E0382`.
// error: could not compile `l1-5`

// To learn more, run the command again with --verbose.
// error: could not compile `l1-5`

// To learn more, run the command again with --verbose.
