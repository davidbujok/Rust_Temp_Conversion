fn main() {
    println!("{}", celcius_to_farenheit(40.0));
    println!("{}", farenheit_to_celcius(104.0));
}

fn celcius_to_farenheit(value: f32) -> f32 {
    value * 9.0/5.0 + 32.0
}
fn farenheit_to_celcius(value: f32) -> f32 {
    (value - 32.0) * 5.0/9.0
}
