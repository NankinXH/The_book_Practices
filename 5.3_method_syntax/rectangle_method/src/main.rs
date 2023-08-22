#[derive(Debug)]
struct Rectangle {
    height: u32,
    width: u32,
}
// The currently is to implement the method area of Rectangle
impl Rectangle {
    fn area(&self) -> u32 {
        self.height * self.width
    }
    fn width(&self) -> bool {
        self.width > 0
    }
    fn height(&self) -> bool {
        self.height > 100
    }
    fn can_hold(&self, other_rect: &Rectangle) -> bool {
        self.height > other_rect.height && self.width > other_rect.height
    }
    fn new(width: u32, height: u32) -> Self {
        Self {
            height: width,
            width: height,
        }
    }
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let scale = 2; // This scale is prepare for proving the debug marco.
    let rect1 = Rectangle::new(180, 90);

    let rect2 = Rectangle {
        height: dbg!(scale * 90),
        width: 50,
    };
    if rect2.width() {
        println!("the rectangle's width is existing {}", rect2.width);
    }
    if rect2.height() {
        println!("the rectangle's height is existing {}", rect2.height);
    }

    println!("{}", rect2.area());
    println!("{}", rect2.can_hold(&rect1));
}
