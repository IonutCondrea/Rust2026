fn panic_add(a: u32, b: u32) -> u32 {
    a.checked_add(b).unwrap_or_else(|| panic!("Overflow la adunare"))
}

fn panic_mul(a: u32, b: u32) -> u32 {
    a.checked_mul(b).unwrap_or_else(|| panic!("Overflow la inmultire"))
}

fn main() {
    let suma = panic_add(10, 20);
    println!("Suma: {}", suma);
    
    let produs = panic_mul(5, 5);
    println!("Produs: {}", produs);
}