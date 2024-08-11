fn main() {
    const DAYS: [&str; 12] = [
        "first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth",
        "tenth", "eleventh", "twelfth",
    ];

    const GIFTS: [&str; 12] = [
        "A partridge in a pear tree",
        "Two turtle doves",
        "Three French hens",
        "Four calling birds",
        "Five gold rings",
        "Six geese a-laying",
        "Seven swans a-swimming",
        "Eight maids a-milking",
        "Nine ladies dancing",
        "Ten lords a-leaping",
        "Eleven pipers piping",
        "Twelve drummers drumming",
    ];

    for day_index in 0..DAYS.len() {
        println!(
            "On the {} day of Christmas my true love sent to me:",
            DAYS[day_index]
        );

        for gift_index in (0..=day_index).rev() {
            if gift_index == 0 && day_index != 0 {
                println!("And {}", GIFTS[gift_index].to_lowercase());
                // Takes into account the small change on last line for the first day
                // By adding "And" and lowercase the gift text to change the "A" to "a"
            } else {
                println!("{}", GIFTS[gift_index]);
            }
        }
        println!("---"); // Visual separator
    }
}
