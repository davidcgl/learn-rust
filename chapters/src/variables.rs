pub fn run() {
    const MAX_POINTS: u32 = 100_000;
    let x = 1 + MAX_POINTS;
    let mut y = 1 + x;

    println!("The value of x is {}", x);
    println!("The value of y is {}", y);

    // Shadowing x (can also change type)
    let x = "hello";

    // Reassigning y to a different value (only works on mut)
    y = 20;

    println!("The value of x is {}", x);
    println!("The value of y is {}", y);

    // Tuples
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("tup is: {} {} {}", x, y, z);
    println!("tup is: {} {} {}", tup.0, tup.1, tup.2);

    // Arrays (type; length) have fixed size.
    // Vectors are variable size.
    let arr1: [i32; 5] = [1, 2, 3, 4, 5];
    let first = arr1[0];

    // Shorthand for [3, 3, 3, 3, 3]
    let arr2 = [3; 5];
    // Functions
    another_function(10, 32.6);
    println!("plus_one: {}", plus_one(5));
    // Expressions
    let x = {
        let a = 100;
        // Note that expressions don't end in semicolon
        a + 1
    };
    println!("expression: {}", x);
}

// In function signatures, you must declare the type of each parameter. This is a deliberate
// decision in Rust’s design: requiring type annotations in function definitions means the compiler
// almost never needs you to use them elsewhere in the code to figure out what you mean.

fn another_function(x: i32, y: f64) {
    println!("another function: {} {}", x, y);
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
