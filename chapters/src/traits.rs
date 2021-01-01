use std::fmt;
use std::fmt::Display;

pub fn run() {
    let news = NewsArticle {
        headline: String::from("Big News!!"),
        location: String::from("San Francisco"),
        author: String::from("David Chen"),
        content: String::from("This is a big news."),
    };

    let tweet = Tweet {
        username: String::from("davidcgl"),
        content: String::from("this is a tweet"),
        reply: false,
        retweet: false,
    };

    println!("news: {}", news.summarize());

    println!("{}", &tweet);
    println!("{}", get_summary_1(&tweet));
    println!("{}", get_summary_2(&tweet));
    println!("{}", get_summary_3(&tweet));
    println!("{}", combine_summary_1(&news, &tweet));
    println!("{}", combine_summary_2(&news, &tweet));
}

// item must be a type that implements Summary trait
fn get_summary_1(item: &impl Summary) -> String {
    format!("get_summary_1: {}", item.summarize())
}

// Trait bound syntax.
// get_summary_1 is just a syntactic sugar for this
fn get_summary_2<T: Summary>(item: &T) -> String {
    format!("get_summary_2: {}", item.summarize())
}

// Multiple trait bounds
// T must implement `Summary` and `Display`
fn get_summary_3<T: Summary + Display>(item: &T) -> String {
    format!("get_summary_3: {}", item.summarize())
}

// Multiple trait parameters.
fn combine_summary_1<T: Summary, U: Summary>(item1: &T, item2: &U) -> String {
    format!(
        "summary 1: {}\nsummary 2: {}",
        item1.summarize(),
        item2.summarize()
    )
}

// Alternative/cleaner syntax with where clauses
fn combine_summary_2<T, U>(item1: &T, item2: &U) -> String
where
    T: Summary,
    U: Summary,
{
    format!(
        "summary 1: {}\nsummary 2: {}",
        item1.summarize(),
        item2.summarize()
    )
}

// This doesn't compile! If we want to return "impl Summary", our code must only return one concrete
// type (either NewsArticle *or* Tweet, can't be both).
// fn make_summary(username: &str, content: &str, is_news: bool) -> impl Summary {
//     if is_news {
//         NewsArticle {
//             headline: String::from("Big News!!"),
//             location: String::from("San Francisco"),
//             author: String::from(username),
//             content: String::from(content),
//         };
//     } else {
//         Tweet {
//             username: String::from(username),
//             content: String::from(content),
//             reply: false,
//             retweet: false,
//         }
//     }
// }

// Trait specifies a list of methods that must be implemented by any type that implement this trait.
pub trait Summary {
    // This method comes with a default implementation, which can be overriden.
    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }

    // This method must be implemented by all types.
    fn summarize_author(&self) -> String;
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }

    fn summarize_author(&self) -> String {
        self.author.clone()
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }

    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}

impl Display for Tweet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Tweet by @{}: \"{}\"", self.username, self.content)
    }
}

// Using trait bounds to conditionally implement methods
struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    // Self = the implementing type within a trait or impl block, or the current type within a type
    // definition.
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

// This is only implemented for all Pair<T> where T implements Display and PartialOrd
impl<T: Display + PartialOrd> Pair<T> {
    pub fn compare(&self) {
        if self.x < self.y {
            println!("x={} is less than y={}", self.x, self.y);
        } else if self.x > self.y {
            println!("x={} is more than y={}", self.x, self.y);
        } else {
            println!("x={} is equal to y={}", self.x, self.y);
        }
    }
}
