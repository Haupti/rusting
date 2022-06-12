use rusting::fizzbuzz;

#[test]
fn it_returns_zero_if_input_zero() {
    let actual: String = fizzbuzz(0);
    assert_eq!(actual, "0");
}

#[test]
fn it_returns_two_if_input_2() {
    let actual: String = fizzbuzz(2);
    assert_eq!(actual, "2");
}

#[test]
fn it_returns_fizz_on_divisible_by_three() {
    for input in 1..3 {
        let actual: String = fizzbuzz(3 * input);
        assert_eq!(actual, "fizz")
    }
}

#[test]
fn it_returns_buzz_on_divisible_by_five() {
    for input in [1,2,4] {
        let actual: String = fizzbuzz(5 * input);
        assert_eq!(actual, "buzz")
    }
}

