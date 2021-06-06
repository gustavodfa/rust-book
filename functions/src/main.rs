fn main() {
    println!("Hello, world!");

    another_function(32, 2);
    scope_function();
    println!("forty_two() returns {}", forty_two());
    println!("five() returns {}", five());
}

fn another_function(x: i32, y: i32) {
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
}

fn scope_function() {
    let x = 5;

    let y = {
        let x = 3;
        x + 1   // x + 1 doesn't end with `;`, therefore it isn't
                // a statement but an expression. This expression
                // will be bind to y.
    };
    println!("The value of y is: {}", y);
    println!("The value of x is: {}", x);
}

fn forty_two() -> i32 {
    return 42;  // This is an explicit return.
}

fn five() -> i32 {
    5   // If it is stated that a function returns something, but
        // there isn't a explicit return statement, the last expression
        // will be returned. This reminds me of good ol' Ruby.
        //
        // PS.: It MUST be an expression (`5 + 2`), NOT A STATEMENT! (`5;`)
}
