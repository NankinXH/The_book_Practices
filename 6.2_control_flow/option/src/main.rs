fn plus_one(x: Option<i32>) -> i32 {
  // matchers must cover every possible.
    match x {
        None => 0,
        Some(i) => i + 1,
    }
}
fn main() {
  let fifteen = Some(15);
  let sixteen = plus_one(fifteen);
  let none = plus_one(None);
  println!("fifteen: {:?}, sixteen: {:?}, none: {:?}", fifteen, sixteen, none);
}
