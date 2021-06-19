fn main() {
    let fruit = vec!['🥝', '🍌', '🍇'];
    let buffer_overflow = fruit[4];

    assert_eq!(buffer_overflow, '🍉')
}

// thread 'main' panicked at 'index out of bounds: the len is 3 but the index is 4', l1-7/src/main.rs:3:27
// note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
