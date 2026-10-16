fn add_space(s: &mut String, n: u32) {
    let mut i = 0;
    while i < n {
        s.push(' ');
        i += 1;
    }
}

fn add_str(s: &mut String, t: &str) {
    s.push_str(t);
}

fn add_integer(s: &mut String, n: u64) {
    let mut div = 1;
    let mut digits = 1;
    while n / div >= 10 {
        div *= 10;
        digits += 1;
    }

    while div > 0 {
        let d = n / div % 10;
        s.push((d as u8 + b'0') as char);
        digits -= 1;
        if digits > 0 && digits % 3 == 0 {
            s.push('-');
        }
        div /= 10;
    }
}

fn add_float(s: &mut String, n: f64) {

}

fn main() {
    println!("Hello, world!");
}
