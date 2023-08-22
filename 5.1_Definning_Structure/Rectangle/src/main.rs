#[derive(Debug)]
struct Rectangle {
    height: u32,
    width: u32,
}
fn main() {
    let scale = 2;
    let rect1 = Rectangle {
        height: dbg!(177 * scale),
        width: 120,
    };
    println!("{}", area(&rect1));
    println!("rect1 is {:#?}", &rect1);
    // Calling the macro dbg! to print the standard error console stream (stderr), as opposed to println!
    dbg!(rect1);
}
fn area(rect: &Rectangle) -> u32 {
    rect.height * rect.width
}
