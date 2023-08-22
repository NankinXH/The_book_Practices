fn main() {
    let the_word: String = String::from("this is the sample");
    // let result = first_word(&the_word);
    // let the_first_word = &the_word[0..4];
    // let the_second_word = &the_word[5..7];
    let the_first_word = first_word(&the_word);
    // the_word.clear();
    println!("the first word: {}", the_first_word);
}
fn first_word(some_string: &String) -> &str {
    let bytes = some_string.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &some_string[0..i];
        }
    }
    &some_string[..]
}
