pub fn run() {
    // Option<T> is a common enum defined in the standard library
    let x: Option<i32> = Some(100);
    match x {
        None => {
            println!("y is None!")
        }
        Some(val) => {
            println!("y contains {}", val);
        }
    };

    // match expression must be exhaustive.
    // We can use the _ pattern to match any value.
    let x = 100;
    match x {
        1 => println!("one!"),
        2 => println!("two!"),
        _ => println!("did not match any!"),
    }

    // If we only care about one value in pattern matching, use if let expression to be concise
    let x = Some(10);

    // Wordy!
    match x {
        Some(a) => println!("x contains value: {}", a),
        _ => (),
    }

    // Concise! The tradeoff is we lose exhaustiveness checking that comes with `match`
    if let Some(a) = x {
        println!("x contains value: {}", a);
    }

    // if let also can have an else-clause (behaves like _)
    if let Some(a) = x {
        println!("x contains value: {}", a);
    } else {
        println!("x has no value!");
    }
}
