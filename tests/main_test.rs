extern crate rusting;

use rusting::main;

#[cfg(test)]
mod main_test {
    #[test]
    fn it_returns_zero_if_input_zero(){
        let actual: String = main::fizzbuzz(0);
        assert_eq!(actual, "0");
    }
}
