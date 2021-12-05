fn main() {
    say_hello(); // Does not have to be defined before main
    print_integer(5);

    let x = 5; // statement
    let _y = 5 + 6; // '5+6' is an expression

    // Statements cannot be assigned to variables
    // The flollowing is not allowed!
    // let y = (let x = 5);

    // {} (scopes) are expressions
    let y = {
        let x = x + 1;
        x + 1
    };
    print_integer(y);

    println!("4>5? {}", greater_than(4, 5));
}

// Conventional style is snake_case for fn names
fn say_hello() {
    println!("Hello!");
}

// parameter type must be specified in fn signatures
fn print_integer(x: i32) {
    println!("Got {}!", x);
}

fn greater_than(x: i32, y: i32) -> bool {
    if x>y {
        return true // return value does not end with ';'
    }
    false // return keyword is optional at the end of a scope
}