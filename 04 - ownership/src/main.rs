fn main() {
    {
        // _s is not valid here, it’s not yet declared
        let _s = "hello"; // _s is valid from this point forward
    } // this scope is now over, and _s is no longer valid

    // the following will not compile (cannot find value `_s` in this scope)
    // println!("{}", _s)

    let s = String::from("Hello!");

    takes_ownership(s);

    let x = 5;

    makes_copy(x);

    let s = String::from("I need this string");

    let s = returns_ownership(s);

    println!("[main]: I own `{}`", s);

    // referencing is done using &
    // ownership is not passed when using references
    println!("The length of `{}` is {}", s, calc_length(&s));

    // creating a reference is called borrowing. Borrowed references are immutable by default!
    let mut s = String::from("hello");

    add_name(&mut s);
    println!("{}", s);

    // only one mutable borrow can occur at a time
    // The following would fail:
    /*
    let r1 = &mut s;
    let r2 = &mut s;

    println!("{}, {}", r1, r2);
    */

    // Dangling refrences (where the memory refrenced 'goes away') are not allowed in rust.
    // Ownership transfer is used instead in such cases
    let s = no_dangle();
    println!("{}", s);
}

fn takes_ownership(s: String) {
    println!("[takes_ownership]: I took over the string `{}`", s);
}

fn makes_copy(x: i32) {
    println!("[makes_copy]: I copied `{}`", x);
}

fn returns_ownership(s: String) -> String {
    println!(
        "[returns_ownership]: I will be nice and return the ownership of `{}`",
        s
    );
    s
}

fn calc_length(s: &String) -> usize {
    // s is a reference to a String
    s.len()
} // s goes out of scope but is not freed since it is not owned by the reference

fn add_name(s: &mut String) {
    s.push_str(", Rustacean!");
}


fn no_dangle() -> String {
    let s = String::from("I am valid!");

    s
}