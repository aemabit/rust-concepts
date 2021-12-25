// DERIVE
#[derive(Debug)]
struct User {
    name: String,
    age: i32,
}

// // CUSTOM TRAIT DEBUG 
// impl std::fmt::Debug for User {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(f, "User {}, have {} years old", self.name, self.age)
//     }
// }
fn main() {

    let new_user = User {
        name: "Andres Eduardo".to_string(),
        age: 27,
    };

    println!("Hello {:?}", new_user)
}