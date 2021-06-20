fn main() {
    let one = [1, 2, 3];
    let two: [u8; 3] = [1, 2, 3];
    let blank1 = [0; 3];
    let blank2: [u8; 3] = [0; 3];
    let arrays = [one, two, blank1, blank2];

    for a in &arrays {
        print!("{:?}: ", a);
        for n in a.iter() {
            print!("\t{} + 10 = {}", n, n + 10);
        }

        let mut sum = 0;
        for i in 0..a.len() {
            sum += a[i];
        }

        println!("\t(SUM{:?} = {})", a, sum);
    }
}

// [1, 2, 3]: 	1 + 10 = 11	2 + 10 = 12	3 + 10 = 13	(SUM[1, 2, 3] = 6)
// [1, 2, 3]: 	1 + 10 = 11	2 + 10 = 12	3 + 10 = 13	(SUM[1, 2, 3] = 6)
// [0, 0, 0]: 	0 + 10 = 10	0 + 10 = 10	0 + 10 = 10	(SUM[0, 0, 0] = 0)
// [0, 0, 0]: 	0 + 10 = 10	0 + 10 = 10	0 + 10 = 10	(SUM[0, 0, 0] = 0)
