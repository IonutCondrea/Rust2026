#[derive(Debug, PartialEq)]
enum MathError {
    Overflow,
}

fn res_add(a: u32, b: u32) -> Result<u32, MathError> {
    a.checked_add(b).ok_or(MathError::Overflow)
}

fn res_mul(a: u32, b: u32) -> Result<u32, MathError> {
    a.checked_mul(b).ok_or(MathError::Overflow)
}

fn add_then_multiply(a: u32, b: u32, c: u32) -> Result<u32, MathError> {
    let sum = res_add(a, b)?;
    let product = res_mul(sum, c)?;
    Ok(product)
}

fn main() {
    let rezultat = add_then_multiply(2, 3, 4);
    println!("{:?}", rezultat);
}