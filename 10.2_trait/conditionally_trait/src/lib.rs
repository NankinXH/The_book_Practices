use std::fmt::Display;
pub fn add(left: usize, right: usize) -> usize {
    left + right
}

struct Pair<T> {
    x: T,
    y: T,
}
impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y}
    }
}
// Use trait bound to conditionally implement the method
impl<T: Display+ PartialOrd> Pair<T> {
    fn cmp_display(&self) {
       if self.x > self.y  {
        println!("the largest one is x = {}", self.x);
       } else {
        println!("the largest one is y = {}", self.y);
       }
        
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
