fn main() {
    let the_iterator = vec![1, 22, 44];
    let new_iterator = the_iterator.iter().map(|x|x+1);
    println!("{:#?}", new_iterator);
}
