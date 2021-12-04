fn main() {
    // constant cannot be shadowed/calculated in runtime
    const SECONDS_IN_YEAR: usize = 60 * 60 * 24 * 12;
    println!("seconds in a year: {}", SECONDS_IN_YEAR);

    // mutable values can be changed
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    let y = 10; // immutable by default
    println!("The value of y is: {}", y);

    // shadowing: assigning a new value to 'y'. Effectively a new variable
    let y = "Hello";
    println!("The value of y is: {}", y);

    // in _z the '_' indicates that variable won't be used
    let _z: i8 = -128; // signed integer 8-bits (i8..i128) or isize
    let _z: u8 = 255; // unsigned integer 8-bits (u8..u128) or usize

    let _z = 0.56; // f64 by default
    let _z: f32 = 0.25;

    let _f = true; // bool type (1 byte)

    let kitten = '🐈'; // char (4 byte, Unicode)
    println!("kitten: {}", kitten);

    /* COMPOUND TYPES */

    // tuple: fixed length, variety of types
    let _tup = ('O', 'K', "🙆‍♂️", 3.14, 0xfff); // without type annotations
    let tup: (char, char, &str, f32, u128) = ('O', 'K', "🙆‍♂️", 3.14, 0xfff);

    // accessing tuple elements
    let (_, _, ok, _, _) = tup; // destructuring a tuple value
    let num = tup.4; // access by index
    println!("ok: {}, num: {}", ok, num);

    // unit type - unit value
    /* Returned implicitly by expressions if they don't return any other value */
    let _unit = ();

    // array: fixed-length, same type
    let _days = ["Mon", "Tue", "Wed", "Thu", "Fri", "Sat", "Sun"];
    let _a: [i32; 5] = [1, 2, 3, 4, 5]; // with annotation [type; # of elements]
    let _a = [10; 2]; // _a=[10, 10] all elements have the same value [val, # of elements]
}
