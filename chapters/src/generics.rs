pub fn run() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest_i32(&number_list);
    println!("largest_number: {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];
    println!("largest_char: {}", largest_char(&char_list));
    println!("largest_1: {}", largest_1(&char_list));
    println!("largest_2: {}", largest_2(&char_list));

    let point1 = Point { x: 1, y: 2 };
    let point2 = Point { x: 1.0, y: 2.0 };
    println!(
        "point2 distance from origin: {}",
        point2.distance_from_origin()
    );
    let point3 = Point { x: "hello", y: 3.2 };
    let point4 = point3.mixup(point1);
    println!("point4: x={}, y={}", point4.x(), point4.y());
}

struct Point<T, U> {
    x: T,
    y: U,
}

// Methods available on all Point<T, U>
impl<T, U> Point<T, U> {
    fn x(&self) -> &T {
        &self.x
    }

    fn y(&self) -> &U {
        &self.y
    }

    // Methods can be generic too!
    fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

// Methods only available on Point<f32, f32>
impl Point<f32, f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

fn largest_i32(list: &[i32]) -> &i32 {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn largest_char(list: &[char]) -> &char {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn largest_1<T: PartialOrd + Copy>(list: &[T]) -> T {
    // We need Copy to copy item (bitwise) into largest
    let mut largest = list[0];
    for &item in list {
        // We need PartialOrd for `>`
        if item > largest {
            // We need Copy to copy item (bitwise) into largest
            largest = item;
        }
    }
    largest
}

// Another way we could implement largest is for the function to return a reference to a T value in
// the slice. If we change the return type to &T instead of T, thereby changing the body of the
// function to return a reference, we wouldn’t need the Clone or Copy trait bounds and we could avoid
// heap allocations.
fn largest_2<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}
