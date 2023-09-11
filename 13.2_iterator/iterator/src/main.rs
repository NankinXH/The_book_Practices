fn main() {
    let list = vec![32,43,55];

    let itered_list = list.iter();
    for i in itered_list {
        println!("current item: {}", i);
    } 

}
