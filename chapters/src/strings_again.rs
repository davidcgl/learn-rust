pub fn run() {
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    // format! is a macro like println! except it returns a string
    let s4 = format!("{}-{}-{}", s1, s2, s3);
    println!("s4: {}", s4);

    // String concatenation transfers ownership!
    let s5 = String::from("hello");
    let s6 = String::from("world");
    // + uses the add method:
    // fn add(self, s: &str) -> String
    //
    // Notes:
    //   - Since add() takes a self (not &self), it takes over the ownership
    //   - s6 is a String, but it is "deref coerced" into a &str
    let s7 = s5 + " " + &s6;
    println!("s7: {}", s7);

    // Using s5 here is invalid because it is moved to s7!
    // println!("{}", s5);

    // String is a wrapper over Vec<u8>. They represent the raw bytes that represent the UTF-8
    // strings. String doesn't support indexing like s[0] because some scalar values are multi-byte,
    // and it is not clear what s[0] should return (first byte? first Unicode scalar value? first
    // grapheme cluster?).

    // However, string slicing with range works. But if the range index doesn't make sense like
    // &s[0..1] for a multi-byte character, then Rust will panic at runtime.
    let s8 = String::from("hello");
    let r8 = &s8[0..2];
    println!("r8: {}", r8);

    // An alternative is get(), which is similar to &s[i..j] except it returns
    // None if it'd have panicked.
    if let Some(r8) = s8.get(0..2) {
        println!("r8: {}", r8);
    }

    // String iteration
    for c in s8.chars() {
        println!("{}", c);
    }

    // Byte iteration
    for b in s8.bytes() {
        println!("{}", b);
    }
}
