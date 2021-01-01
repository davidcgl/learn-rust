pub fn run() {
    vectors();
}

enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

fn vectors() {
    // Need type annotation because the initial list is empty.
    let v1: Vec<i32> = Vec::new();

    // Inferred type is Vec<i32>. vec! is a standard macro
    let mut v2 = vec![1, 2, 3];

    v2.push(4);
    v2.push(5);

    let third: &i32 = &v2[2];
    println!("third element is: {}", third);

    // get() returns Option <&T>
    if let Some(third) = v2.get(2) {
        println!("third element is: {}", third);
    }
    // Iterating over vector
    for i in &v2 {
        println!("{}", i);
    }
    // Mutating vectors
    println!("v2 before: {:?}", v2);
    for i in &mut v2 {
        // * is a dereferencing operator
        *i += 10
    }
    println!("v2 after: {:?}", v2);

    // Using vector to store different values: use enums!
    // Vec<SpreadsheetCell>
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
    for cell in &row {
        match cell {
            SpreadsheetCell::Int(i) => println!("Cell has an integer: {}", i),
            SpreadsheetCell::Text(s) => println!("Cell has a string: {}", s),
            SpreadsheetCell::Float(f) => println!("Cell has a float: {}", f),
        }
    }
    // Slicing
    let v3 = vec![1, 2, 3, 4, 5];
    let v4 = &v3[1..3];
    println!("v4 is {:?}", v4);
    // Passing vector slices is the idiomatic way to pass a read-only Vec to functions, similar to
    // the relationship of String and &str.
    print_slice(&v3);
}

fn print_slice(slice: &[i32]) {
    println!("slice is {:?}, len: {}", slice, slice.len());
}
