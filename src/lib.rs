pub fn main() {
    println!("Hello, world!");
}

pub fn fizzbuzz(x: i32) -> String {
    let is_divisible_by_3 = x % 3 == 0;

    if x != 0 && is_divisible_by_3 {
        return String::from("fizz");
    }
    return format!("{x}")
}
