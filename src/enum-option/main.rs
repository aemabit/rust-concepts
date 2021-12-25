#[allow(dead_code)]
/*
 * ENUM OPTION
*/

// enum Option<T> {
//     Some(T),
//     None,
// }

struct User {
    name: String,
    age: Option<i32>,
}

impl User {
    fn get_age(&self) -> i32 {
        self.age.unwrap_or_default()
    }
}

fn main() {
    // let name: Option<T> = None;

    let name: Option<String> = Some(String::from("Andres"));

    match name {
        None => println!("No name"),
        Some(name) => println!("The name is {}", name),
    }

    let new_user = User {
        name: "Andres Mejias".to_string(),
        age: Some(12),
    };

    let age = new_user.get_age();
    println!("edad: {}", age);
}
