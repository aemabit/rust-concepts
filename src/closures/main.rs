// CLOSURES
// are inline functions

fn main() {

    let sum = |x: i32| -> i32 {
        x + 1
    };
    println!("total sum is: {}", sum(4));

    let counter = 1;

    let increment = move  || {
        counter + 1
    };

    let another_var =  increment(); // Borrowing

    println!("{}", another_var);

}

