#[derive(Debug)]
struct Rectangle {
    height: i32,
    width: i32,
}
impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}
fn greeting_name(name: &str) -> String{
    format!("Hello {}!", name)
    
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn larger_can_hold_smaller() {
        let rect_1 =  Rectangle {
            height: 30,
            width: 40
        };
        let rect_2 = Rectangle {
            height: 20,
            width: 30,
        };
        assert!(rect_1.can_hold(&rect_2));
        // assert!(rect_2.can_hold(&rect_2));
    }
    #[test]
    fn greeinng_contains_name() {
        let res = greeting_name("James");
        assert!(res.contains("James"));
    }
    #[test]
    fn greeinng_not_contains_name() {
        let res = greeting_name("James");
        assert!(res.contains("Carol"), "Greeting did not contain name, value was {}", res);
    }
}
