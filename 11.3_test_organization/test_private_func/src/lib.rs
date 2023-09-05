pub fn adder(left: usize, right: usize) -> usize {
    left + right
}
fn internal_adder(left:usize, right: usize)-> usize {
    left+ right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = adder(2, 2);
        assert_eq!(result, 4);
    }
    #[test]
    fn internal() {
        assert_eq!(4, internal_adder(2, 2));
    }
}
