// ITERATORS

struct  Counter {
    count: i32,
}

impl Counter {
    fn new() -> Counter {
        Counter { count: 0}
    }
}

impl Iterator for Counter {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.count < 5 {
            self.count += 1;
            Some(self.count)
        }else{
            None
        }
    }
}

fn main() {

    let s = [1,2,3];
    for x in s.iter() {
        println!("{}", x+1);
    };
    println!("{:?}", s);

    let mut c = Counter::new();
    c.next();
    c.next();
    c.next();
    c.next();
    c.next();
    c.next();
    let i = c.next();

    match i {
        Some(count) => println!("{}", count),
        None => ()
    }
}

