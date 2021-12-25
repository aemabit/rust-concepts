#[allow(dead_code)]

/*
 * THE STRUCTS AND ENUMS
*/

struct User {
    name: String,
    email: String,
    age: i32,
    active: bool,
    country: String,
    user_role: UserRoles,
    website: Website,
}

impl User {
    fn year_of_birth(&self) -> i32 {
        let actual = 2021;
        return actual - self.age;
    }
}

enum UserRoles {
    BASIC,
    ADMIN,
}

enum Website {
    URL(String),
    INSTAGRAM(String),
    LINKEDIN(String),
    FACEBOOK(String),
}

fn main() {
    let role = UserRoles::BASIC;

    let user = User {
        name: "Andres".to_string(),
        email: String::from("aemabit@gmail.com"),
        age: 27,
        active: true,
        country: String::from("United States"),
        user_role: role,
        website: Website::INSTAGRAM(String::from("@aemabit")),
    };

    println!("The user is: {}, age:{}", &user.name, &user.age);

    // SHORTHAND INIT
    let build_user = new_user(user);

    println!(
        "The user information is: {} has {} living in {} and his email is {}",
        &build_user.name, &build_user.age, &build_user.country, &build_user.email
    );

    let second_user = User {
        name: String::from("Eduardo"),
        ..build_user
    };

    let build_user_two = new_user(second_user);
    println!(
        "The second user information is: {} has {} living in {} and his email is {}",
        &build_user_two.name, &build_user_two.age, &build_user_two.country, &build_user_two.email
    );

    // TUPLE STRUCTS

    struct Point(i32, i32, i32);
    let point_a = Point(12, 32, 56);
    println!("coords #0: {:?}", point_a.0);

    // USE IMPLEMENTATION
    let year_of_birth = User::year_of_birth(&build_user_two);
    println!("year of birth of user #2: {:?}", year_of_birth);

    // HAS ACCESS
    let user_has_access = has_access(build_user_two.user_role);
    println!("User has access: {:?}", user_has_access);

    if user_has_access {
        go_to_website(build_user_two.website);
    } else {
        println!("User can go open the website");
    }
}

fn new_user(user: User) -> User {
    User {
        name: user.name,
        email: user.email,
        age: user.age,
        active: user.active,
        country: user.country,
        user_role: user.user_role,
        website: user.website,
    }
}

fn has_access(user_role: UserRoles) -> bool {
    match user_role {
        UserRoles::ADMIN => true,
        UserRoles::BASIC => false,
    }
}

fn go_to_website(website: Website) {
    match website {
        Website::FACEBOOK(x) => println!("User go to Fb and the user is {}", x),
        Website::INSTAGRAM(x) => println!("User go to Instagram and the user is {}", x),
        Website::LINKEDIN(x) => println!("User go to Linkedin and the user is {}", x),
        Website::URL(x) => println!("User go to Url {}", x),
    }
}
