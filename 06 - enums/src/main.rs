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
    println!("Hello, world!");
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
}
