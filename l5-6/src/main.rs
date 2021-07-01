#[allow(arithmetic_overflow)]
fn main() {
    let (a, b) = (200, 200);
    let c: u8 = a + b;
    println!("200 + 200 = {}", c);
}

// thread 'main' panicked at 'attempt to add with overflow', l5-6/src/main.rs:4:17
// note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
