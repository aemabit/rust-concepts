// Control Flow

fn main() {
    // IF
    let number = 5;

    if number == 5 {
        println!("is -> {}", number);
    } else if number == 3 {
        println!("not is five is -> {}", number);
    } else {
        println!("not is five and not is three is ->  {}", number);
    }

    // INLINE IF
    let result = if number > 5 { 1000 } else { 0 };
    println!("the result is ->  {}", result);

    // LOOP
    let mut counter = 0;
    let result = loop {
        println!("is the {} loop", counter);
        if counter == 10 {
            // here break the loop and return counter
            break counter;
        }
        counter += 1
    };

    println!("result {}", result);

    // WHILE
    let mut count_while = Some(0);

    while let Some(i) = count_while {
        if i == 10 {
            count_while = None;
        } else {
            println!("{}", i);
            count_while = Some(i + 1);
        }
    }

    // FOR
    let array = [0, 1, 2, 3, 4, 5];
    for element in array.iter() {
        println!("element -> {}", element);
    }

    // IF LET
    let age: Option<i32> = Some(20);

    // with match
    match age {
        Some(value) => println!("Age is -> {}", value),
        _ => (),
    }

    // with if let
    if let Some(value) = age {
        println!("Age is -> {}", value);
    }

    // WHILE LET
    let mut unread_message = Some(10);

    // with loop
    // loop {
    //     match unread_message {
    //         Some(value) => {
    //             if value > 0 {
    //                 println!("You have unread messages, {}", value);
    //                 unread_message = Some(value - 1);
    //             } else {
    //                 println!("Don't have new messages");
    //                 unread_message = None;
    //             }
    //         }
    //         _ => {
    //             break;
    //         }
    //     }
    // }

    // with while let
    while let Some(value) = unread_message {
        if value > 0 {
            println!("You have unread messages, {}", value);
            unread_message = Some(value - 1);
        } else {
            println!("Don't have new messages");
            unread_message = None;
        }
    }
}
