use rusting::fizzbuzz;

#[test]
fn it_returns_zero_if_input_zero() {
    let actual: String = fizzbuzz(0);
    assert_eq!(actual, "0");
}

#[test]
fn it_returns_fizz_on_divisible_by_three() {
    for input in (1..3) {
        let actual: String = fizzbuzz(3 * input);
        assert_eq!(actual, "fizz")
    }
}
