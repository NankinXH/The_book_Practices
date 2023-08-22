fn main() {
    let point_to_nothing = dangling();
    println!("current point : {}", point_to_nothing);
}
fn dangling()-> String {
    let the_possible_dangling_variable = String::from("the dangling string");
    the_possible_dangling_variable
}