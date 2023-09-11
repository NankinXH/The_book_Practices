fn main() {
    let the_iterator = vec![1, 22, 44];
    let new_iterator = the_iterator.iter().map(|x|x+1);
    println!("{:#?}", new_iterator);
    let new_new_vec:Vec<_> = new_iterator.map(|x|x+1).collect();
    println!("{:?}",new_new_vec);
}

