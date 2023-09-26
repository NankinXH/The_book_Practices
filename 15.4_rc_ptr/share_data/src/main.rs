// use std::fmt;
use std::rc::Rc;

#[derive(Debug)]
enum List {
    Cons(i32, Rc<List>),
    Nil,
}
use crate::List::{Cons, Nil};

// impl fmt::Display for List {
//     fn fmt(&self, f: &mut fmt::Formatter)->fmt::Result {
//         write!(f, "{}->{}", self)
//     }
// }

fn main() {
    let z = Rc::new(Cons(10, Rc::new(Nil)));

    let a = Rc::new(Cons(5, Rc::clone(&z)));

    let b = Cons(3, Rc::clone(&a));

    let c = Cons(4, Rc::clone(&a));

    println!("{:#?}, \n{:#?},\n{:#?}, \n{:#?}", a, b, c, z);
}
