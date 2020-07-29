mod fibonacci;
mod temperature_conversion;

fn main() {
    temperature_conversion::convert_temperatures();
    let answer = fibonacci::fibonacci(12);
    println!("fibonacci of 12 is {}", answer);
}
