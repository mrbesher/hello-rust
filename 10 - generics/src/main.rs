#![allow(unused)]
use std::fmt::Debug;
use std::fmt::Display;
// monomorphization is applied where all generic
// types are replaced with concrete ones not leading
// to slowdown in runtime

// a generic type `T` is defined here, allowing
// any type in the struct
struct Point<T> {
    x: T,
    y: T,
}

// when a struct allows multiple generic types
// it can be defined as below
struct Rect<T, U> {
    w: T,
    h: U,
}

// methods can be implemented as follows for
// generic types
impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

// methods can be restricted to some types only
impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt() // sqrt(x^2 + y^2)
    }
}

// LIB CODE
// traits tell the compiler about a functionality of
// a particular type
pub trait Summary {
    // instead of the function signture below
    // a default implementation can be provided
    fn summarize_author(&self) -> String;

    // default implementation can call other methods
    // even if those don't have a default impl
    fn summarize(&self) -> String {
        format!("Read more from {}...", self.summarize_author())
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize_author(&self) -> String {
        format!("{}", self.author)
    }

    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }

    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

// functions can be implemented on types implementing a trait
pub fn notify<T: Summary>(item: &T) {
    println!("Breaking! {}", item.summarize());
}

// another syntax using `impl`. multiple traits can be specified
// using `+`
pub fn send_mail(item: &(impl Summary + Display)) {
    println!("Did you check out this? {}", item.summarize());
}

// sometimes trait bounds can be hard to read `where` syntax is
// an alternative in such cases
fn some_function<T, U>(t: &T, u: &U) -> impl Summary
where
    T: Display + Clone,
    U: Clone + Debug,
{
    let tweet = Tweet {
        username: String::from("Snowden"),
        content: String::from("Can you hear me now?"),
        reply: false,
        retweet: false,
    };

    // we can return any type implementing Summary but
    // the same function cannot have cases returning a
    // different type based on execution
    tweet
}

// 'a specifies that all variables annotated with it must
// live at least as long as each others live
// this is necessary since the string returned is not known
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// a function with generics and lifetime
fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
where
    T: Display,
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    // `x` and `y` must have the same type
    let p1 = Point { x: 5, y: 3 };
    println!("p.x = {}", p1.x());

    // any type is allowed since not traits
    // are specified
    let p2 = Point { x: 3.5, y: -1.01 };
    println!(
        "p2 is {:.2} units away from origin",
        p2.distance_from_origin()
    );

    // `w` and `h` can have different or similar types
    let rect = Rect { w: 3.4, h: 1 };
    let rect = Rect { w: 3.4, h: 1.01 };
    let rect = Rect { w: 4, h: 2 };

    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from(
            "The Pittsburgh Penguins once again are the best \
             hockey team in the NHL.",
        ),
    };

    println!("New Article! {}", article.summarize());

    // liftimes
    {
        let r;

        {
            let x = 5;
            r = &x;
        }

        // the following won't compile since x is out of
        // scope (doesn't live long enough)
        // println!("r: {}", r);
    }

    // 'static liftime store variables in the program's binary
    // check if the variable really has to live through all the
    // program lifetime before using 'static
    let s: &'static str = "static lifetime!";

    let q1 = String::from("Even a smile can be charity");
    {
        let q2 = String::from("Two truths cannot contradict one another");
        println!("The longest phrase: `{}`", longest(&q1, &q2));
    }

    // we cannot call `longest` here with `q2` since it is out of scope
    // longest(&q1, &q2);
}
