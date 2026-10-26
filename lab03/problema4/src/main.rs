#[derive(Debug, PartialEq)]
enum CharError {
    NotAscii,
    NotDigit,
    NotBase16,
    NotLetter,
    NotPrintable,
}

fn to_uppercase(c: char) -> Result<char, CharError> {
    if !c.is_alphabetic() { return Err(CharError::NotLetter); }
    Ok(c.to_ascii_uppercase())
}

fn to_lowercase(c: char) -> Result<char, CharError> {
    if !c.is_alphabetic() { return Err(CharError::NotLetter); }
    Ok(c.to_ascii_lowercase())
}

fn print_char(c: char) -> Result<(), CharError> {
    if c.is_control() { return Err(CharError::NotPrintable); }
    println!("{}", c);
    Ok(())
}

fn char_to_number(c: char) -> Result<u32, CharError> {
    if !c.is_ascii() { return Err(CharError::NotAscii); }
    c.to_digit(10).ok_or(CharError::NotDigit)
}

fn char_to_number_hex(c: char) -> Result<u32, CharError> {
    if !c.is_ascii() { return Err(CharError::NotAscii); }
    c.to_digit(16).ok_or(CharError::NotBase16)
}

fn print_error(e: CharError) {
    match e {
        CharError::NotAscii => println!("Caracterul nu este ASCII"),
        CharError::NotDigit => println!("Caracterul nu este o cifra"),
        CharError::NotBase16 => println!("Caracterul nu este o cifra in baza 16"),
        CharError::NotLetter => println!("Caracterul nu este o litera"),
        CharError::NotPrintable => println!("Caracterul nu este printabil"),
    }
}

fn main() {
    let c = 'f';

    match char_to_number_hex(c) {
        Ok(val) => println!("Valoare hex: {}", val),
        Err(e) => print_error(e),
    }

    let _ = to_uppercase('a');
    let _ = to_lowercase('A');
    let _ = print_char('x');
    
    let eroare_cifra = char_to_number('a');
    if let Err(e) = eroare_cifra {
        print_error(e);
    }
}