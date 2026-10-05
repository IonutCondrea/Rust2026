fn coprim(a: i16, b: i16) -> bool {
    let (mut x, mut y) = (a, b);
    while y != 0 {
        let r = x % y;
        x = y;
        y = r;
    }

    if x == 1 {
        return true;
    }

    false
}

fn main() {
    let mut i = 0;
    while i <= 100 {
        let mut j = 0;
        while j <= 100 {
            if coprim(i, j) {
                println!("({}, {}) sunt coprime", i, j);
            }
            j += 1;
        }
        i += 1;
    }
}
