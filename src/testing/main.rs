// Test -> unit tests
// #[test] -> cargo test
// cargo test `sum` (keywords)

// #[should_panic] -> must throw a panic
// #[should_panic(expected = "some expected text")]

// #[ignore] -> with this ignore the test when try cargo test
fn main() {

    // assert!(expression) -> 
    // assert_eq!(a, b) -> values are equal
    // assert_ ne!(a,b) -> values arent equal\
}

fn sum (a: i32, b: i32) -> i32 {
    a + b
}

fn only_number(code: &str) -> bool {
    code.chars().all(char::is_alphanumeric)
}


#[test]
fn check_code_with_numbers() {
    let result = only_number("91191902");
    assert!(result, "The code has letters, code used: {}", "91191902")
}

#[test]
fn sum_good() {
    assert_eq!( sum(2,2), 4);
}
