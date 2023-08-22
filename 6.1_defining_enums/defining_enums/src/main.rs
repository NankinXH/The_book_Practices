#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32),
}

// Implement the Message enum.
impl Message {
    fn call(&self) {
        println!("{:#?}", &self)
    }
}
fn main() {
    let m = Message::Write(String::from("hello"));
    m.call();
}
