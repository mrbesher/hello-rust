#[derive(Debug)]
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

impl std::fmt::Display for User {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "is_active: {}, username: {}, email: {}, sign_in_count: {}",
            self.active, self.username, self.email, self.sign_in_count
        )
    }
}

// tuple structs
struct Point(i32, i32, i32);

// unit-like structs
struct AlwaysEqual;

fn main() {
    let mut user1 = User {
        email: String::from("besher@example.com"),
        username: String::from("beshr"),
        sign_in_count: 1,
        active: false,
    };

    // since the struct is mutable, its fields can be modified
    // the entire instance is either mutalbe or immutable
    user1.username = String::from("besher");

    let user2 = create_user("besher@example.com".to_string(), "besher".to_string());

    // other fields not explicitly set should have the same values
    // as in user1
    let user3 = User {
        email: String::from("besher@example.org"),
        ..user1
    };

    println!("{}", user3);
    println!("{:#?}", user2);
    dbg!(user2); // prints to stderr

    // The following would fail! Since user1 is partially moved
    // to user3 not copied.
    // println!("{}", user1);

    let _p = Point(0, 0, 0);
    let _u = AlwaysEqual; // will be used to define traits (in later chapters)
}

fn create_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}
