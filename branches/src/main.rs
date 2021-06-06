fn main() {
    let number = 3;
    if number != 0 {  // condition MUST result in boolean.
        println!("number was three");
    }

    let condition = true;
    let number = if condition { 5 } else { 6 };  // This works!

    println!("The value of number is: {}", number);


    loop {  // this is an infinite loop.
        println!("again!");
        break;  // breaking so I don't need to send an interrupt signal.
    }

    let mut counter = 0;
    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;  // this is weird, but count * 2 will
                                // be the return value of the loop.
                                // YES, loops can have return values :O
        }
    };
    println!("The value of result is: {}", result);
}
