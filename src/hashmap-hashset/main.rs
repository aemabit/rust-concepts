use std::collections::{HashMap, HashSet};

// HASHMAP
fn main() {
    let mut points: HashMap<String, i32> = HashMap::new();
    let black = String::from("Black");
    let black_points = 22;
    points.insert(black, black_points);
    points.insert(String::from("Blue"), 20);
    points.insert(String::from("Red"), 10);
    points.insert(String::from("Yello"), 22);

    let blue_pointes = points.get("Blue");

    match blue_pointes {
        Some(value) => println!("points of blue -> {}", value),
        _ => println!("Blue team not exist"),
    }

    points.entry(String::from("White")).or_insert(100);

    println!("All teams");
    for (key, value) in &points {
        println!("{} -> {}", key, value);
    }

    // when black is inserted in the hashmap rust remove the variable (borrowing)
    // println!("black, {}", black); -> this show the error
    let black_points = points.get("Black");

    match black_points {
        Some(value) => println!("points of black -> {}", value),
        _ => println!("Black team not exist"),
    }

    hashset_fn()
}

// HASHSET

fn hashset_fn() {
    println!("HASHSET");
    // the values inside on hashset are uniques
    let mut user_ids: HashSet<i32> = HashSet::new();
    user_ids.insert(100);
    user_ids.insert(82);
    user_ids.insert(88);
    user_ids.insert(910);
    // remove
    // user_ids.remove(&910);

    let mut backup_user: HashSet<i32> = HashSet::new();
    backup_user.insert(100);
    backup_user.insert(82);
    backup_user.insert(88);
    backup_user.insert(91);
    backup_user.insert(910);

    // union
    // difference
    for id in backup_user.difference(&user_ids) {
        println!("diffrence is: {}", id)
    }

    // intersenticon
    // symentric_difference

    for id in user_ids.iter() {
        println!("id: {}", id);
    }
}
