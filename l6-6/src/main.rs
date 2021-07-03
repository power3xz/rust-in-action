fn main() {
    let a: i64 = 42;
    let a_ptr = &a as *const i64;
    println!("a: {}, ({:p})", a, a_ptr);
}

// a: 42, (0x7ffee2932110)
