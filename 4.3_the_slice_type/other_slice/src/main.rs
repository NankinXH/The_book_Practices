fn main() {
    let a_number_array = [1,2,3,4,5];
    let slice = &a_number_array[1..3];
    assert_eq!(slice, &[2,3]);
    println!("{:?}",  slice); 
}
