fn main() {
    // If you have a mutable reference to a value already, you can have no other reference to that value.
    let mut the_string = String::from("the string about anything");
    // {
    //     let the_something_1 = &mut the_string;
    //     println!("the inner one {}", the_something_1);
    // }
    // let the_something_2 = &mut the_string; // Here is the error occur.
    // println!("the outter one {}", the_something_2);
    let pointer_one = &the_string;
    let pointer_two = &the_string;
    println!("pointer_one {}, pointer_two {}", pointer_one, pointer_two);
    // the pointer_one and pointer_two cannot be used after the println excute.
    let pointer_mutable = &mut the_string;
    println!("pointer_mutable: {}", pointer_mutable);
}
