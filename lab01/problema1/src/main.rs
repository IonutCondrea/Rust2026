fn prim(n: i16) -> bool {
    let mut x: i16 = 3;
    if n == 1 {
        return false;
    } else if n == 2 {
        return true;
    } else if n % 2 == 0 {
        return false;
    }

    while x * x <= n {
        if n % x == 0 {
            return false;
        }
        x += 2;
    }
    true
}

fn main() {
    let mut numar: i16 = 0;
    while numar <= 100 {
        if prim(numar) {
            println!("{} ", numar);
        }
        numar += 1;
    }
}
