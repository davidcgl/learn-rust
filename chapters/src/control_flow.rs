pub fn run() {
    let x = 100;

    // If expressions (only works on bool, no implicit type conversion)
    if x < 0 {
        println!("negative");
    } else if x == 0 {
        println!("zero");
    } else {
        println!("positive");
    }

    // If is an expression!
    // Each arm must evaluate to the same type (i32 in this case)!
    let number = if x < 0 { 5 } else { 6 };
    println!("x: {}, number: {}", x, number);

    // loop runs until break
    // can return a value
    let mut counter = 0;
    let number = loop {
        println!("counter: {}", counter);
        if counter == 3 {
            break counter;
        }
        counter += 1;
    };
    println!("number: {}", number);

    // For-in loops is preferred for most use cases
    // (0..5) is a Range type
    for i in 0..5 {
        println!("i: {}", i)
    }
    let arr = [1, 2, 3];
    for j in arr.iter() {
        println!("j: {}", j);
    }
}
