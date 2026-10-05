fn main() {
    let mut beer: i16 = 99;

    while beer > 0 {
        println!("{} bottles of beer on the wall,", beer);
        println!("{} bottles of beer.", beer);
        println!("Take one down, pass it around");
        if beer == 1 {
            println!("No bottles of beer on the wall.");
        } else {
            println!("{} bottles of beer on the wall.", beer - 1)
        };
        println!();
        beer -= 1;
    }
}
