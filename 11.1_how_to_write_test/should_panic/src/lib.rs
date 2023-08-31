struct Guess {
    value: i32
}
impl Guess {
    fn new(value:i32)-> Self{
        if value > 100 || value<1 {
            panic!("Guess value must be less than the 100 and greater than 1 !")
        }
        Guess { 
            value
         }
    }
    fn new_det(value:i32)-> Self {
        if value > 100 {
            panic!("Guess value must be greater than or equal to 1, got {}.", value);
        }else if value < 1 {
            panic!("Guess value must be greater than or equal to 1, get {}.", value);
        }
        Self { value }
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn guess_value_greater_100() {
       Guess::new(200);
    }
    #[test]
    #[should_panic("expected: less than or equal to  100")]
    fn guess_greater_100() {
        Guess::new_det(101);
    }
}
