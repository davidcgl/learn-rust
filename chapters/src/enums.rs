enum IpAddrKind {
    V4(u8, u8, u8, u8),
    V6(String),
}

// Defining an enum with variants such as Message below is similar to defining different kinds of
// struct definitions, except the enum doesn’t use the struct keyword and all the variants are
// grouped together under the Message type.
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

pub fn run() {
    let home = IpAddrKind::V4(127, 0, 0, 1);
    let loopback = IpAddrKind::V6(String::from("::1"));
}
