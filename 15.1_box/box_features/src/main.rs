use crate::List::{Cons, Nil};
#[derive(Debug)]
enum List {
    Cons(i32, Box<List>),
    Nil,
}
fn main() {
    let b = Box::new(8);
    println!("b = {}", b);

    let cur_list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    println!("{:#?}", cur_list);
}
