pub fn run() {
    let x = String::from("hello");
    borrow_ownership(&x);
    // x still owns "hello"

    get_ownership(x);
    // x no longer owns "hello"
    // "hello" is moved to get_ownership
    let mut y = String::from("world");
    borrow_mut_ownership(&mut y);
    println!("y is now: {}", y)
}

fn get_ownership(s: String) {
    println!("I own `s` now: {}", s)
}

fn borrow_ownership(s: &String) {
    println!("I borrowed `s`: {}", s)
}

fn borrow_mut_ownership(s: &mut String) {
    s.push_str(", david");
    println!("I borrowed `mut s`: {}", s)
}
