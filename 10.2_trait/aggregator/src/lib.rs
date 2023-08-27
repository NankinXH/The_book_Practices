pub fn add(left: usize, right: usize) -> usize {
    left + right
}
pub trait Summary {
    fn summarize_author(&self) -> String;
    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}
pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}
// impl Summary for NewsArticle {
//     fn summarize_author(&self)-> String {
//         self.author
//     }
//     // fn summarize(&self) -> String {
//     //     format!("{}, by {} ({})", self.headline, self.author, self.location)
//     // }
// }
pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}
impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
    // fn summarize(&self) -> String {
    //     format!("{}: {}", self.username, self.content)
    // }
}
// This parameter specified impl keyword accept any type that implements the trait before.
pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize())
}
// The impl trait syntax works for straightforward cases but actually syntax sugar for a longer form known as trait bound.
pub fn notify1<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}
// The longer form to using the form.
pub fn notify_long_one<T: Summary>(item1: &T, item2: &T) {
    println!(
        "Breaking news! {}, {}",
        item1.summarize(),
        item2.summarize()
    )
}
// // We could use"+" signature to define both Summary and Display;
// pub fn notify_display(item: &(impl Summary + Display)) {}
// // We could also use the shorter form in this situation;
// pub fn notify_display_one<T: Summary + Display>(item: &T) {}

// Sometimes, the shorter form of the trait bound looks so long.
fn some_function_zero<T: Clone + Debug, U: Display + Clone>(t: &T, u: &U) {}
// Using where clauses to define clearer trait bound;
// fn some_function_one<T, U>(t: &T, u: &U) -> i32
// where
//     T: Clone + Debug,
//     U: Display + Clone,
// {
// }
// Here are the codes that we return the types which implement the summary trait without naming the concrete type name.
// 
fn returns_summarizable(switch: bool) -> impl Summary {
    if switch {
        NewsArticle {
            headline: String::from(
                "Penguins win the Stanley Cup Championship!",
            ),
            location: String::from("Pittsburgh, PA, USA"),
            author: String::from("Iceburgh"),
            content: String::from(
                "The Pittsburgh Penguins once again are the best \
                 hockey team in the NHL.",
            ),
        }
    } else {
        Tweet {
            username: String::from("horse_ebooks"),
            content: String::from(
                "of course, as you probably already know, people",
            ),
            reply: false,
            retweet: false,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::Summary;

    use super::{notify, NewsArticle, Tweet};

    #[test]
    fn it_works() {
        let tweet = Tweet {
            username: String::from("horse ebook"),
            content: String::from("of course, as you"),
            reply: false,
            retweet: false,
        };
        println!("{}", tweet.summarize());

        let article = NewsArticle {
            headline: String::from("Penguins win the Stanley Cup Championship!"),
            location: String::from("Pittsburgh, PA, USA"),
            author: String::from("Iceburgh"),
            content: String::from(
                "The Pittsburgh Penguins once again are the best \
                 hockey team in the NHL.",
            ),
        };
        notify(&tweet);

        // println!("an article: {}", article.summarize());
    }
}
