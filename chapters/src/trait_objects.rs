pub fn run() {
    let screen = Screen {
        components: vec![
            Box::new(Button {
                width: 10,
                height: 20,
                label: String::from("Click me!"),
            }),
            Box::new(TextField {
                text: String::from("Hello world"),
            }),
        ],
    };
    screen.run();
}

pub trait Draw {
    fn draw(&self);
}

// A trait object points to both an instance of a type implementing our specified trait as well as
// a table used to look up trait methods on that type at runtime. We create a trait object by
// specifying some sort of pointer, such as a & reference or a Box<T> smart pointer, then the dyn
// keyword, and then specifying the relevant trait.
//
// This works differently from defining a struct that uses a generic type parameter with trait
// bounds. A generic type parameter can only be substituted with one concrete type at a time,
// whereas trait objects allow for multiple concrete types to fill in for the trait object at
// runtime.
//
// When we use trait objects, Rust must use dynamic dispatch. The compiler doesn’t know all the
// types that might be used with the code that is using trait objects, so it doesn’t know which
// method implemented on which type to call. Instead, at runtime, Rust uses the pointers inside the
// trait object to know which method to call.
pub struct Screen {
    // components can holds any structs that implements Draw. This is different than a generic
    // Vec<Box<T>> where T: Draw, because the latter can only hold structs *of the same type* that
    // implements Draw.
    pub components: Vec<Box<dyn Draw>>,
}

impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}

pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

impl Draw for Button {
    fn draw(&self) {
        println!("Button: ({}, {}) {}", self.width, self.height, self.label);
    }
}

pub struct TextField {
    pub text: String,
}

impl Draw for TextField {
    fn draw(&self) {
        println!("TextField: {}", self.text);
    }
}
