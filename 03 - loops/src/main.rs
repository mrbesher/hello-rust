fn main() {
    // will not stop unless told to
    loop {
        println!("Endless?");
        break; // breaks from loop
    }

    let mut count = 0;
    // Loop label
    'counting_up: loop {
        println!("count = {}", count);
        let mut remaining = 10;

        loop {
            println!("remaining = {}", remaining);
            if remaining == 9 {
                // breaks to innermost loop
                break;
            }
            if count == 2 {
                // breaks to the loop with label counting_up
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {}", count);

    count = 0;
    let result = loop {
        count += 1;

        if count == 10 {
            // returns expression after break
            break count * 2;
        }
    };

    println!("result = {}", result);

    // while loop
    while count > 0 {
        count -= 1;
    }

    // iterate through a collection
    let a = [10, 20, 30, 40];
    for elem in a {
        println!("element: {}", elem);
    }

    // countdown with style!
    // (1..4) a range with 1 included and 4 excluded
    for number in (1..4).rev() {
        println!("{}", number);
    }
}
