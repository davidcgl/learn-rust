pub fn run() {
    structs();
    tuple_structs();
    more_structs();
}

struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn build_user(username: &str, email: &str) -> User {
    User {
        email: String::from(email),
        username: String::from(username),
        sign_in_count: 1,
        active: true,
    }
}

fn structs() {
    // Immutable user
    let u1 = build_user("user1", "user1@example.com");
    println!("user 1: {}", u1.username);
    // Mutable user
    let mut u2 = build_user("user2", "user2@example.com");
    u2.username = String::from("newuser2");
    println!("user 2: {}", u2.username);
    // Struct shorthand
    let username = String::from("user3");
    let email = String::from("user3@example.com");
    // Note `username: username` becomes `username`
    let u3 = User {
        username,
        email,
        sign_in_count: 1,
        active: true,
    };
    // Struct update syntax
    // The syntax .. specifies that the remaining fields not explicitly set should have the same
    // value as the fields in the given instance.
    let u4 = User {
        username: String::from("user4"),
        email: String::from("user4@example.com"),
        ..u3
    };
}

// Tuple structs
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn tuple_structs() {
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
}

// More structs

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // &self is a &Rectangle aka immutable ref to a Rectangle
    fn area(&self) -> u32 {
        self.width * self.height
    }

    // Take &mut self if we want mutation
    fn update_width(&mut self, width: u32) {
        self.width = width;
    }
    // Functions within impl blocks that don't take `self` is called an associated function.
    // They are associated with the struct (Rectangle), like static methods in other languages
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

fn more_structs() {
    let mut r1 = Rectangle {
        width: 30,
        height: 50,
    };
    // r1 must be mutable for this to compile.
    r1.update_width(20);

    // This requires Rectangle to have #[derive(Debug)]
    // {:?} to print debug info in single line
    // {:#?} to print debug info in multiline
    println!("Rectangle is: {:?}", r1);
    println!("The area is {} square pixels", r1.area());
    let r2 = Rectangle::square(10);
    println!("r2 is a square: {:?}", r2);
}
