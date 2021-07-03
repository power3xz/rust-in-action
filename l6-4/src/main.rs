use std::mem::size_of;

static B: [u8; 10] = [99, 97, 114, 114, 121, 116, 111, 119, 101, 108];
static C: [u8; 11] = [116, 104, 97, 110, 107, 115, 102, 105, 115, 104, 0];

fn main() {
    let a: usize = 42;
    let b: &[u8; 10] = &B;
    let c: Box<[u8]> = Box::new(C);

    println!("a (an unsigned integer):");
    println!("  location: {:p}", &a);
    println!("  size:     {:?} bytes", size_of::<usize>());
    println!("  value:    {:?}", a);
    println!();

    println!("b (an refernce to B):");
    println!("  location: {:p}", &b);
    println!("  size:     {:?} bytes", size_of::<&[u8; 10]>());
    println!("  value:    {:p}", b);
    println!();

    println!("c (a \"box\" for C):");
    println!("  location: {:p}", &c);
    println!("  size:     {:?} bytes", size_of::<Box<[u8]>>());
    println!("  value:    {:p}", c);
    println!();

    println!("B (an array of 10 bytes):");
    println!("  location: {:p}", &B);
    println!("  size:     {:?} bytes", size_of::<[u8; 10]>());
    println!("  value:    {:?}", B);
    println!();

    println!("C (an array of 11 bytes):");
    println!("  location: {:p}", &C);
    println!("  size:     {:?} bytes", size_of::<[u8; 11]>());
    println!("  value:    {:?}", C);
    println!();
}

// a (an unsigned integer):
//   location: 0x7ffeecd06a78
//   size:     8 bytes
//   value:    42

// b (an refernce to B):
//   location: 0x7ffeecd06a80
//   size:     8 bytes
//   value:    0x102f34be0

// c (a "box" for C):
//   location: 0x7ffeecd06a88
//   size:     16 bytes
//   value:    0x7fa5e8c05ce0

// B (an array of 10 bytes):
//   location: 0x102f34be0
//   size:     10 bytes
//   value:    [99, 97, 114, 114, 121, 116, 111, 119, 101, 108]

// C (an array of 11 bytes):
//   location: 0x102f34bea
//   size:     11 bytes
//   value:    [116, 104, 97, 110, 107, 115, 102, 105, 115, 104, 0]
