use std::collections::HashMap;

fn main() {
    let vec_integers: Vec<i32> = vec![1, 2, 3, 4, 6, 6];
    let vec_integers_odd: Vec<i32> = vec![1, 2, 3, 4, 5];
    let vec_integers_even: Vec<i32> = vec![1, 2, 3, 4, 5, 6];

    println!("Median (odd): {:?}", median(vec_integers_odd));
    println!("Median (even): {:?}", median(vec_integers_even));

    println!(
        "The integer with the most appearances is: {:?}",
        mode(vec_integers)
    );
}

fn median(mut vector: Vec<i32>) -> f32 {
    vector.sort();
    let vector_length: usize = vector.len();

    if vector_length % 2 == 1 {
        vector[vector_length / 2] as f32
    } else {
        let mid1: i32 = vector[vector_length / 2 - 1];
        let mid2: i32 = vector[vector_length / 2];
        (mid1 + mid2) as f32 / 2.0
    }
}

fn mode(vector: Vec<i32>) -> i32 {
    let mut appearances: HashMap<i32, i32> = HashMap::new();

    for integer in &vector {
        let count: &mut i32 = appearances.entry(*integer).or_insert(0);
        *count += 1;
    }

    let mut max_count = 0;
    let mut mode = vector[0];

    for (&key, &value) in appearances.iter() {
        if value > max_count {
            max_count = value;
            mode = key;
        }
    }

    mode
}
