/*
 * THE FUNCTION HAS THE CONVENTION SNAKE_CASE
*/

fn main() {
    show_greetings();
    select_number(8);
    let sum_value = return_sum_value(&5);
    println!("the addition total is: {}", sum_value);
    generic_greetings("Andres");
}

fn show_greetings() {
    println!("Welcome to Rust");
}

fn select_number(number: i32) {
    println!("The number is {}", number);
}

fn return_sum_value(value: &i32) -> i32 {
    *value + 10
}

fn generic_greetings(name: &str) {
    println!("Hello {}!", name)
}   