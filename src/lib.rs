pub fn main() {
    println!("Hello, world!");
}

pub fn fizzbuzz(x: i32) -> String {
    let is_divisible_by_3 = x % 3 == 0;
    let is_divisible_by_5 = x % 5 == 0;

    if x == 0 {
        return format!("{x}")
    }

    if is_divisible_by_3 {
        return String::from("fizz");
    }

    if is_divisible_by_5 {
        return String::from("buzz");
    }

    return format!("{x}")
}
