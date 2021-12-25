// TRAITS = rasgo
struct Human;
struct Cat;

trait Talk {
    fn say_hi(&self) -> String;
    fn lang() -> String {
        "I can't talk".to_string()
    }
}

impl Talk for Human {
    fn say_hi(&self) -> String {
        "Hello guys!".to_string()
    }
    fn lang() -> String {
        "Talk like an Human".to_string()
    }
}

impl Talk for Cat {
    fn say_hi(&self) -> String {
        "Miau!".to_string()
    }
    fn lang() -> String {
        "Talk like a Cat".to_string()
    }
}

trait DriverLicence {
    fn is_ready_to_drink(&self) -> bool;
}

impl DriverLicence for Option<i32> {
    fn is_ready_to_drink(&self) -> bool {
        match self {
            Some(age) => {
                if age > &18 {
                    true
                } else {
                    false
                }
            }
            _ => false,
        }
    }
}

fn main() {
    let andres = Human;
    println!("{}", andres.say_hi());
    println!("{}", Human::lang());

    let kity = Cat;
    println!("{}", kity.say_hi());
    println!("{}", Cat::lang());

    let my_age = Some(20);
    if my_age.is_ready_to_drink() {
        println!("Common give me a beer for this guy!!!!")
    } else {
        println!("Fu*k you! and come back with your mom!!!")
    }
}
