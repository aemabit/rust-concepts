/*
 * CONSTANTS are declared in MAYUS and are inmutables.
*/
const CONSTANT_VARIABLE: &str = "CONSTANT_VALUE";

fn main() {
    /*
     * Rust needs know is the varaible is mutable if not for default is inmutable.
     */
    mutable_and_redeable();
    /*
     * Rust track the variable value until it runs/used.
     */
    shadowing();

    println!("I'm constant value: {}", CONSTANT_VARIABLE);
}

fn shadowing() {
    let spacing = "   ";

    println!("Spacing are {} this", spacing);

    let spacing = spacing.len();

    println!("Spacing are {} this", spacing)
}

fn mutable_and_redeable() {
    let mut mutable_variable = "initial";

    println!("Initial Mutable variable is: {}", mutable_variable);

    let redeable_variable = "value";
    mutable_variable = redeable_variable;

    println!("Mutable variable is: {}", mutable_variable);
    println!("Redeable varaible is: {}", redeable_variable);
}
