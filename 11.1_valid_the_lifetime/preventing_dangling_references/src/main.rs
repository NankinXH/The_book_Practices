fn main() {
    let one;
    // {
    let two = 6;
    // The variable two cannot live long enough.
    one = &two;
    // }
    println!("one: {}", one);
    let the_first = String::from("the first");
    // let the_second = "the second";
    // let the_maximum = longest(the_first, the_second);
    // println!("the largest str is the {}", the_maximum);

    let result;
    {
        let the_second = String::from("the second");
        result = longest(the_first.as_str(), the_second.as_str());
    }
    println!("The longest string is {}", result);
}

//&i32        // a reference
// &'a i32     // a reference with an explicit lifetime
// &'a mut i32 // a mutable reference with an explicit lifetime

fn longest<'a, 'b>(x: &'a str, y: &'b str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
fn str_longest<'a>(x: &str, y: &str) -> &'a str {
    let actually_longest = String::from("the actually long str");
    actually_longest.as_str()
}
