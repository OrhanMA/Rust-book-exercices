fn main() {
    let x = fahreinheit_to_celsius(32.0);
    println!("32 fahreinheit to celsius is {x}");

    let y = celsuis_to_fahreinheit(0.0);
    println!("0 celsuis to fahreinheit is {y}");
}

fn fahreinheit_to_celsius(value: f32) -> f32 {
    (value - 32.0) * 5.0/9.0
}

fn celsuis_to_fahreinheit(value: f32) -> f32 {
    (value * 9.0/5.0) + 32.0
}