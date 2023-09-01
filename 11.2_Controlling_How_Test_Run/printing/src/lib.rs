fn prints_returns_10(a: i32) -> i32 {
    println!("I got a value {}", a);
    10
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn this_test_will_pass() {
        let value = prints_returns_10(10);
        assert_eq!(10, value);
    }
    #[test]
    fn this_test_will_fail() {
        let value = prints_returns_10(6);
        assert_eq!(5, value);
    }
}
