// COLLECTIONS
fn main() {
    // VECTOR
    let mut v: Vec<i32> = Vec::new();

    v.push(9);

    vector_iteration();
}

fn vector_iteration() {
    let mut v2 = vec![1, 2, 3];
    v2.push(100);

    let third = v2.get(2);

    if third.is_some() {
        println!("Third Element: {}", third.unwrap());
    }

    // or

    match third {
        Some(value) => println!("Third Element: {}", value),
        _ => println!("nothing here"),
    }

    // loop vector

    for i in &v2 {
        println!("value is: {}", i);
    }

    for i in &mut v2 {
        *i+=10
    } 

    for i in &v2{
        println!("mutable value is: {}", i);
    }

    enum Message {
        TEXTO(String),
        ERROR(i32),
    }


    let messages = vec![Message::TEXTO("hello world".to_string()), Message::ERROR(404)];

    for m in &messages {
       match m {
           Message::TEXTO(texto) =>  println!("text: {}", texto),
           Message::ERROR(error) => println!("error: {}", error)
       }
    }
}
