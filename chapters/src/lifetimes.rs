use std::fmt::Display;

// Every reference &T has a lifetime, which is the scope for which that reference is valid.
pub fn run() {
    compiles();
    does_not_compile();

    structs();
}

fn compiles() {
    // this function compiles because res does not outlive s2
    let s1 = String::from("abcd");
    {
        let s2 = String::from("xyz");
        let res = longest(s1.as_str(), s2.as_str());
        println!("compiles: {}", res);
    }
}

fn does_not_compile() {
    // this function does not compile if uncommented because res outlives s2 and is used after s2
    // goes out of scope. It will compile if we don't use it, though (remove println!).

    // let s1 = String::from("abcd");
    // let res;
    // {
    //     let s2 = String::from("xyz");
    //     res = longest(s1.as_str(), s2.as_str());
    // }
    // println!("compiles: {}", res);
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// Let’s briefly look at the syntax of specifying generic type parameters, trait bounds, and
// lifetimes all in one function!
fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
where
    T: Display,
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// This annotation means an instance of ImportantExcerpt can’t outlive the reference it holds in
// its part field.
struct ImportantExcerpt<'a> {
    part: &'a str,
}

fn structs() {
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let mut excerpt = ImportantExcerpt {
        part: first_sentence,
    };

    // This block does not compile because excerpt outlives &hello[0..2]
    // {
    //     let hello = String::from("hello");
    //     excerpt.part = &hello[0..2];
    // }

    println!("excerpt.part: {}", excerpt.part);
}
