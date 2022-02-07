#![allow(unused)]

// collections point to data stored on the heap

fn main() {
    // vectors store items of the same type next
    // to each other in memory
    // // https://doc.rust-lang.org/std/vec/struct.Vec.html
    let v: Vec<i32> = Vec::new();
    // type annotation is required above since
    // data type is not known intially.
    // The `vec!` macro creates a new vector holding
    // the values we give it. Rust can infer the type
    // in such cases which is `i32` here.
    {
        let v = vec![1, 2, 3];
    } // v and all its elements are freed when out of scope

    // Rust can infer the type since later an int
    // is pushed to the vector
    let mut v = Vec::new();

    // we use the push method to add elements
    v.push(1);
    v.push(6);
    v.push(7);

    // directly access by index and reference program
    // will crash if index is out of bound
    let second = &v[1];
    println!("The second element is {}", second);

    // remove last element
    v.pop();
    // remove element in an index
    v.remove(0);

    v.push(2);

    // the following won't compile since borrowing and
    // ownership rules do not allow for mut and immut reference
    // at the same time. location of elements can be moved when
    // allocating memory so the reference can be invalid.
    // println!("The second element is {}", second);

    // the `get` method can be used to ensure that the
    // program won't panic in case of out-of-bound access
    match v.get(2) {
        Some(third) => println!("The third element is {}", third),
        None => println!("No third element exists."),
    }

    // iterating over values in a vector
    // mutable references
    for i in &mut v {
        *i *= 10;
    }

    // immutable references
    for i in &v {
        println!("{}", i);
    }

    // enums can be used to cluster multiple item types
    // as elements of a vector
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    // strings can be created using the `new` function
    let s = String::new();

    // or using `to_string` method available on any type
    // implementing the Display trait
    let s = "It is much better to be feared than loved.".to_string();
    println!("{}", s);

    // or using the `from` function. Strings are UTF-8 encoded
    let s = String::from("المغلوب مولع دائماً بتقليد الغالب");
    println!("{}", s);

    // modifying strings
    let mut s = String::from("No one can stand up against the authority of truth");

    // strings can be expanded by one char using push
    s.push(',');

    // or by a string using `push_str`
    s.push_str(" and the evil of falsehood is to be fought");

    let s2 = String::from(" with enlightening speculation.");

    // `+` uses the `add` method defined similar to the following
    // fn add(self, s: &str) -> String {
    // the ownership of the first string is therefore taken
    let full_quote = s + &s2; // s has moved here and cannot be used
    println!("{}", full_quote);

    // the format! macro can be used to make things clearer
    let s1 = String::from("Taking a new step");
    let s2 = String::from("uttering a new word");
    let s3 = String::from("is what people fear most");

    let s = format!("{}, {}, {}.", s1, s2, s3);
    println!("{}", s);
    // referencing a char by index is invalid in Rust!
    // since the size of each char is variable
    // let c = &s[0]

    // UTF-8 can be thought of as bytes, scalar values
    // and grapheme clusters (letters for humans)
    // https://en.wikipedia.org/wiki/UTF-8
    // string slices can be used to instead. this can cause
    // panic if the value slices doesn't have valid chars
    let t = &s1[0..1];

    // iterating over Unicode scalar values is possible
    // using the `chars` method
    let s = String::from("Çünkü ben kitap değilim, çünkü ben öldükten sonra kimse beni okuyamaz");
    for c in s.chars() {
        match c {
            ' ' => break,
            other => println!("{}", other),
        }
    }
    println!("{}", s);

    // bytes can be iterated using `bytes` method. Getting grapheme
    // clusters is complex and not supported by std library

    // Hashmap<K, V> map a key K of some type to a value V of some type
    // Hashmap is not brought into scope by default
    use std::collections::HashMap;

    // Hashmap uses SipHash by default, different hashers can be specified
    // by the programmer. A lot are avaliable on crates.io
    // https://en.wikipedia.org/wiki/SipHash
    let mut scores = HashMap::new();
    scores.insert(String::from("Black"), 3);

    // Hashmap can be constructed using collect too
    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];

    // Rust can infer the type which we didn't specify
    // and used `_` instead
    let mut scores: HashMap<_, _> = teams.into_iter().zip(initial_scores.into_iter()).collect();

    // get returns Some(T) or None if the key is not present
    let b_score = scores.get("Blue");
    match b_score {
        Some(score) => println!("{}", score),
        None => (),
    }

    // HashMap can be iterated as follows
    for (key, value) in & scores {
        println!("{}: {}", key, value);
    }

    // updating a Hashmap can be done in three ways
    // overwriting old value
    scores.insert(String::from("Blue"), 40);

    // insert if no value exists `or_insert` method
    scores.entry(String::from("Black")).or_insert(30);


    // update based on the current value
    let text = String::from("War is peace");

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);
}
