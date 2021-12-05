fn main() {
    let x = 5;

    // condition must be a bool
    if x > 5 {
        println!("Greater than five!");
    }

    // The following is not allowed
    // if x {
    //  /* do something */
    // }

    // Using if in a let statement
    let _num = if x > 5 { x } else { 6 };
}
