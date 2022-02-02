fn main() {
    // a slice lets you reference a contiguous sequence of elements in a collection

    let s = String::from("'Forty-two,' said Deep Thought, with infinite majesty and calm");

    // string slices can be created by specifying the range
    // [starting_index..ending_index]
    let said = &s[13..17];
    let deep = &s[18..22];

    // starting and ending indices can be omitted
    let fortytwo = &s[..12]; // [0..12]
    let rest = &s[23..]; // [23..len]

    println!("{} {} {} {}", fortytwo, said, deep, rest);

    let word = last_word(&s);
    println!("Last word: `{}`", word);
}

fn last_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    let l = bytes.len();

    for (i, &item) in bytes.iter().rev().enumerate() {
        if item == b' ' {
            return &s[l-i..];
        }
    }

    &s[..]
}
