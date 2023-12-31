use std::ops::Deref;

#[derive(Debug)]
struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x:T) -> MyBox<T> {
        MyBox(x)
    }
    
}

impl<T> Deref for MyBox<T> {
    type Target =  T;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn main() {
    let x = 5;
    let y = MyBox::new(5);

    
    println!("{}",*(y.deref()));
    assert_eq!(5, x);
    assert_eq!(5, *y);

    println!("{:#?}", y);
    
}
