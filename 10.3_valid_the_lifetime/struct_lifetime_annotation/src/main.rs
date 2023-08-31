#[derive(Debug)]
// The annotation means the instance of ImportantExcerpt can't outlive the reference it holds in its part field.
struct ImportantExcerpt<'a> {
    part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
    fn level(&self)->i32{
        3
    }
    fn announce_and_return_part(&self, announcement:& str)->&str {
        println!("Attention please: {}", announcement);
        self.part
    }
}
fn main() {
    let novel = String::from("Call me Ishmael. Some years ago...");
    println!("the first word: {}", first_word(novel.as_str()));
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let the_excerpt = ImportantExcerpt {
        part: first_sentence,
    };
    println!("{:#?}", the_excerpt);
}
// The lifetime annotation is elision because the 
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}
