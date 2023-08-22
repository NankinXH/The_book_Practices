fn main() {
    let mut the_string = String::from("this is a piece of mutable reference");
    change_size(&mut the_string);
    println!("{}", the_string);
}

fn change_size(something: &mut String) {
    something.push_str(" and borrowing sample");
}
