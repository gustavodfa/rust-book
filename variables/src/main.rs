fn main() {
    let mut x = 5;  // x is mut(able) so it works.
    println!("The text value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);
    
    let spaces = "   ";
    let spaces = spaces.len();  // first `spaces` is shadowed by new `spaces`,
                                // that's why this is valid!
}
