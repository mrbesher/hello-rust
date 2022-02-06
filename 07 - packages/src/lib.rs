#![allow(unused)]

// modules let us organize code in crates for easy reuse
// and control the privacy of items

// the semicolumn afte mod name tells Rust to load mod contents
// from another file with the same name as the module
mod front_of_house;

mod back_of_house {
    pub struct Breakfast {
        // some fields can be pub while others are private
        // in structs. `pub` must be explicitly used for
        // each field
        pub toast: String,
        seasonal_fruit: String,
    }

    // enums variants are all public or private
    pub enum Appetizer {
        Soup,
        Salad,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }
}

// paths may be very long at times. A path can be brought into scope
// and its items can be used as if they are local via the `use` keyword
use crate::front_of_house::hosting;
// or using a relative path
// use self::front_of_house::hosting;

// bringing the function itself to the scope is possible but unidiomatic
// since when calling the function we want to make it clear it's not
// locally defined.
use crate::front_of_house::hosting::add_to_waitlist;

// here is an example for the idiomatic way brining std lib's HashMap
// struct into the scope of the binary crate. The full path is specified
// when brining in structs, enums and other items idiomatically
use std::collections::HashMap;

// some items may have the same name, while the full path can be avoided
// in such cases using `as` may be handy too
use std::fmt::Result;
use std::io::Result as IoResult;

// re-exporting names can be useful to make the path we specify by use
// available to external code. This may come in handy when other
// programmers would think think differently about the domain and the
// internal structure of your code. This can be accomplished by `pub use`
pub use crate::hosting::remove_from_waitlist;

// nested paths can be used to bring multiple items into scope in one line
// self here brings std::io into scope
use std::io::{self, Write};
// * brings all public items defined in a path
use std::collections::*;

pub fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();

    // Order a breakfast in the summer with Rye toast
    let mut meal = back_of_house::Breakfast::summer("Rye");
    // Change our mind about what bread we'd like
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    // The next line won't compile if we uncomment it; we're not allowed
    // to see or modify the seasonal fruit that comes with the meal
    // meal.seasonal_fruit = String::from("blueberries");

    // public enum variants
    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;

    // since hosting is brought into scope above we can use one of its
    // items as follows
    hosting::add_to_waitlist();
}
