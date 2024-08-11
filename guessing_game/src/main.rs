use std::io;
// input/output library - comes from the standard library std
use std::cmp::Ordering;
// Enum type - comes from the standard library std
use rand::Rng;
// the Rng trait - defines methods that rng (random number generators implements)
fn main() {
    println!("Guess the number!");

    let secret_number: u32 = rand::thread_rng()
        .gen_range(1..=100);

    // println!("The secret number is: {secret_number}");

    loop {
        println!("Please input your guess.");
        // variable are immutable by default in Rust, that's why you have to declare the guess variable using the mut keyword, that stands for mutable.
        // the = sign is used to bound (assign) to a variable
        // the :: syntax indicates an associated function of a T type (here, String type)
        let mut guess: String = String::new();
        // making that variable mutable allows to change its value (empty string) to the user input collected with the next line 
    
        // The & indicates that this argument is a reference, which gives you a way to let multiple parts of your code access one piece of data without needing to copy that data into memory multiple times.
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
    
        // Using again guess as the variable name is called shadowing and is useful to convert a value to a different type without creating a new variable with a distinct name.
        // we match returned value of the parse  to a u32 to decide what should we done in case of Ok / Err
        let guess: u32 = match guess.trim().parse() {
            // note that Ok and Err are the two variants (paths) of the Result enum returned by the parse function
            Ok(num) => num, // just returns (forwards the number) - so here let guess equals num
            Err(_) => {
                println!("The guess should be a number!");
                continue;
            }, // continue the loop in case of invalid input, meaning it will ask the user again
            // the _ is a catchall value, meaning all Err will match no matter what is the information inside it contains
        };
        // trim delete whitespace and the newline character caused by the user pressing enter on his keyboard.
        // parse convert the string to the type declared next to the variable name, here u32 for small positive number
    
        println!("You guessed: {}", guess);
        // The {} set of curly brackets is a placeholder
    
        // returns the code associated with the pattern that match the compared values
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                println!("You win!");
                break; // breaks the infinite loop
            },
        }
    }

}