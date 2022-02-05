#![allow(unused)]

enum IpAddrKind {
    V4,
    V6,
}

enum IpAddr {
    // putting data directly into each enum variant is possible too
    V4(String),
    V6(String),
}

// enum variants with different types
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

// enums can have methods too
impl Message {
    fn call(&self) {
        // method body would be defined here
    }
}

// since rust does not have nulls Option enum is used
// to indicate if a value is absent at the moment
/*
enum Option<T> {
    // the variants None and Some can be used
    // without the Option:: prefix
    None,
    Some(T),
}
*/

struct IpAddrStruct {
    kind: IpAddrKind,
    address: String,
}

fn main() {
    let home = IpAddrStruct {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let loopback = IpAddrStruct {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };

    // IpAddr::V4() is a function call that takes a String argument
    // and returns an instance of the IpAddr. A constructor!
    let home = IpAddr::V4(String::from("127.0.0.1"));

    let loopback = IpAddr::V6(String::from("::1"));

    // the Option enum examples
    let absent_number: Option<i32> = None;
    let some_string = Some("a string");

    println!("{}", value_in_cents(Coin::Nickel));

    // using Option<T>
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    // _ indicates that the value is to be ignored
    // empty tuple () means that nothing will be done
    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => (),
    }

    // if let takes a pattern and expression seperated by `=`
    let coin = Coin::Quarter(UsState::Alabama);
    // less typing but losing the exhaustive checking of match
    if let Coin::Quarter(state) = coin {
        println!("State quarter from {:?}!", state);
    }
}

fn value_in_cents(coin: Coin) -> u8 {
    // match is used to compare a value against a series of patterns
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    // match can be used to get the inner value of Some
    match x {
        // matches are exhaustive (omitting None would cause compile-time error)
        // non-exhaustive patterns: `None` not covered
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn add_fancy_hat() {}
fn remove_fancy_hat() {}
