pub fn run() {
    let s = String::from("hello world");
    let len = s.len();
    // Create a string slice
    let a = get_string_slice(&s);
    let b = &s[..5];
    let c = &s[6..len];
    let d = &s[6..];
    println!("a: {}, b: {}, c: {}, d: {}", a, b, c, d);
}

// &str is a string slice
fn get_string_slice(s: &String) -> &str {
    &s[0..5]
}
