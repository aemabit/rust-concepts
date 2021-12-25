// TRAIT DISPLAY
struct User {
    name: String,
    age: i32,
}

// CUSTOM TRAIT DISPLAY 
impl std::fmt::Display for User {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{} ({})" , self.name, self.age)
    }
}
fn main() {

    let new_user = User {
        name: "Andres Eduardo".to_string(),
        age: 27,
    };

    println!("Hello {}", new_user)
}