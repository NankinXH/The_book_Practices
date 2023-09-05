pub fn add_two(a: i32) -> i32 {
    a + 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_2_2() {
        let result = add_two(2);
        assert_eq!(4, result);
    }
    #[test]
    fn add_3_2() {
        let result = add_two(3);
        assert_eq!(5, result);
    }
    #[test]
    #[ignore]
    fn the_100_2(){
        let result = add_two(100);
        assert_eq!(102, result);
    }
    
}
