fn get_first_char(s: &str) -> Option<char> {
    s.chars().next()
}

fn parse_first_char_as_digit(s: &str) -> Result<u32, String> {
    let c = get_first_char(s).ok_or_else(|| String::from("String gol"))?;
    
    if !c.is_ascii() {
        return Err(String::from("Eroare ASCII"));
    }
    
    c.to_digit(10).ok_or_else(|| String::from("Nu este cifra"))
}

fn main() {
    let test_str = "5ani";
    let rez = parse_first_char_as_digit(test_str);
    println!("{:?}", rez);
}