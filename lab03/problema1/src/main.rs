fn is_prime(n: u16) -> bool {
    if n <= 1 { return false; }
    for i in 2..=(n as f32).sqrt() as u16 {
        if n.is_multiple_of(i) { return false; }
    }
    true
}

fn next_prime(x: u16) -> Option<u16> {
    let mut current = x.saturating_add(1);
    while current > x && current < u16::MAX {
        if is_prime(current) {
            return Some(current);
        }
        current += 1;
    }
    if current == u16::MAX && is_prime(current) {
        return Some(current);
    }
    None
}

fn main() {
    let mut numar_curent = 0;
    while let Some(prim) = next_prime(numar_curent) {
        println!("{}", prim);
        numar_curent = prim;
        if numar_curent > 100 {
            break;
        }
    }
}
