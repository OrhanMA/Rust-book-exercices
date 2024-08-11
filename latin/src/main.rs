// Convert strings to pig latin. The first consonant of each word is moved to the end of the word and ay is added, so first becomes irst-fay. Words that start with a vowel have hay added to the end instead (apple becomes apple-hay). Keep in mind the details about UTF-8 encoding!

fn main() {
    let apple = String::from("apple");
    let first = String::from("first");
    let empty = String::from("");
    let single_char = String::from("a");

    println!("{:?}", pig_latin(&apple)); // "apple-hay"
    println!("{:?}", pig_latin(&first)); // "irst-fay"
    println!("{:?}", pig_latin(&empty)); // ""
    println!("{:?}", pig_latin(&single_char)); // "a-hay"
}

fn pig_latin(word: &String) -> String {
    let vowels: &str = "aeiouAEIOU";
    let first_letter: Option<char> = word.chars().next();
    // next() goes to the next iterator so here it's the letter at index 0

    match first_letter {
        Some(c) if vowels.contains(c) => format!("{}-hay", word),
        Some(c) => format!("{}-{}ay", &word[1..], c),
        None => word.to_string(),
    }
}
