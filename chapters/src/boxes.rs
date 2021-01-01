use std::collections::VecDeque;
use std::fmt::Debug;
use std::ops::Deref;

pub fn run() {
    let cons = vec_to_cons((1..5).into_iter().collect::<VecDeque<i32>>());
    println!("cons: {:?}", cons);

    let a = 5;
    let b = &a;
    let c = Box::new(a);
    let d = MyBox::new(a);

    assert_eq!(5, a);
    assert_eq!(5, *b);
    assert_eq!(5, *c); // Box<T> can be dereferenced because it implements Deref
    assert_eq!(5, *d); // MyBox<T> can be dereferenced because it implements Deref

    // Here we’re calling the hello function with the argument &m, which is a reference to a
    // MyBox<String> value. Because we implemented the Deref trait on MyBox<T>, Rust can turn
    // &MyBox<String> into &String by calling deref. The standard library provides an
    // implementation of Deref on String that returns a string slice, and this is in the API
    // documentation for Deref. Rust calls deref again to turn the &String into &str, which matches
    // the hello function’s definition.
    let m = MyBox::new(String::from("Rust"));
    hello(&m);

    // We can also invoke `drop` manually
    // Calling n.drop() (from Drop trait) is a compilation error!
    // Must use std::mem:drop included in prelude
    let n = MyBox::new(String::from("Manual Drop"));
    drop(n);
    println!("before program ends");
}

// The Box<T> type is a smart pointer because it implements the Deref trait, which allows Box<T>
// values to be treated like references. When a Box<T> value goes out of scope, the heap data that
// the box is pointing to is cleaned up as well because of the Drop trait implementation.
//
// We need to use Box<T> for List because it is a recursive data type and has "infinite" size. The
// size of every type must be known at compile time. We break the infinite recursion using a smart
// pointer Box<T>. Box<T> has a size of `usize` because it only holds a pointer. The size of an
// enum is the max size of any of its variants.
#[derive(Debug)]
pub enum List {
    Cons(i32, Box<List>),
    Nil,
}

pub fn vec_to_cons(mut deque: VecDeque<i32>) -> List {
    match deque.pop_front() {
        None => List::Nil,
        Some(i) => List::Cons(i, Box::new(vec_to_cons(deque))),
    }
}

struct MyBox<T: Debug>(T);

impl<T> MyBox<T>
where
    T: Debug,
{
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T>
where
    T: Debug,
{
    type Target = T;

    // Behind the scenes, Rust calls *(x.deref()).
    // The reason deref returns &T is because we don't want the value to be moved out of `self`
    fn deref(&self) -> &T {
        &self.0
    }
}

impl<T> Drop for MyBox<T>
where
    T: Debug,
{
    // drop is called when MyBox<T> goes out of scope
    fn drop(&mut self) {
        println!("dropping MyBox<T> with data {:?}", self.0);
    }
}

pub fn hello(s: &str) {
    println!("Hello {}!", s)
}
