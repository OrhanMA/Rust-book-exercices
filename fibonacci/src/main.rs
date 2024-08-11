use std::io;

fn main() {
    let mut repetitions: String = String::new();

    println!("Please input the number of repetitions desired.");
    io::stdin()
        .read_line(&mut repetitions)
        .expect("Failed to read line");

    let repetitions: i32 = match repetitions.trim().parse() {
        Ok(num)=> {
            println!("Let's go with {num} repetitions!");
            num
        },
        Err(_) => {
            println!("Please enter a valid number.");
            return;
        }
    };

    let (mut x, mut y) = (0, 1);

    for _ in 0..repetitions {
        println!("{}", x);
        let z = x + y;
        // reassign the tuple values in each iteration
        x = y;
        y = z;
    }
}
