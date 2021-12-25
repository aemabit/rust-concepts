use std::ops::Deref;

// Smart pointers = punteros inteligentes
fn main() {
  // Defer trait: hacer posible la dereferenciación (*)
  let x = 5;
  let y = MyBox::new(x);

  if x == 5 {
    println!("hello");
  }
  if *y == 5 {
    println!("hello");
  }
}

struct MyBox<T>(T);

impl<T> MyBox<T> {
  fn new(x:T) -> MyBox<T> {
    MyBox(x)
  }
}

impl<T> Deref for MyBox<T> {
  type Target = T;

  fn deref(&self) -> &Self::Target { 
    &self.0
  }
}
