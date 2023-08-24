fn main() {
    let s1 = String::from("Hello");
    // Rust strings don't support indexing.
    // let h = s1[0];
    let hello = "Здравствуйте";
    // let answer = &hello.chars().nth(0);
    let answer = &hello.bytes().nth(0);
    println!("{:?}", answer);
}
