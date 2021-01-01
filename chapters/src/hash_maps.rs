use std::collections::HashMap;

pub fn run() {
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Gold"), 50);

    // For types that implement the Copy trait, like i32, the values are copied into the hash map.
    // For owned values like String, the values will be moved and the hash map will be the owner of
    // those values.
    let color = String::from("Green");
    let score = 20;
    scores.insert(color, score);

    // color is invalid at this point because it is *moved* into the HashMap

    // Accessing values returns an Option<&V>
    let color = String::from("Blue");
    if let Some(score) = scores.get(&color) {
        println!("score for color {} is {}", color, score);
    } else {
        println!("color {} does not exist!", color);
    }

    // Iteration
    for (color, score) in &scores {
        println!("color: {}, score: {}", color, score);
    }

    // Overwrite a value
    scores.insert(String::from("Blue"), 10);

    // Only insert a value if the key has no value
    scores.entry(String::from("Blue")).or_insert(50);
    println!("blue should still be 10: {}", scores.get(&color).unwrap());

    // Update a value based on the old value
    let text = "hello world wonderful world";
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        // or_insert returns a mutable reference (&mut V) so we can mutate it in-place.
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
}
