fn main() {
    // let s = String::from("hello");

    // takes_ownership(&s);

    // println!("{s}");

    // let x = 5;

    // makes_copy(x);

    let s = String::from("Hello World");
    let word = first_word(&s);
    println!("{}", word);
}

// fn takes_ownership(some_string: &String) {
//     println!("{some_string}");
// }

// fn makes_copy(some_integer: i32) {
//     println!("{some_integer}");
// }

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
