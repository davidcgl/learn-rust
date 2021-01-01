#![feature(or_insert_with_key)]
use std::cmp::Eq;
use std::hash::Hash;
use std::time::Duration;
use std::{collections::HashMap, thread};

pub fn run() {
    generate_workout(10, 3);
}

fn generate_workout(intensity: u32, random_number: u32) {
    // Closures don’t require you to annotate the types of the parameters or the return value like
    // fn functions do (but you can annotate if you want). Type annotations are required on
    // functions because they’re part of an explicit interface exposed to your users. Defining this
    // interface rigidly is important for ensuring that everyone agrees on what types of values a
    // function uses and returns. But closures aren’t used in an exposed interface like this:
    // they’re stored in variables and used without naming them and exposing them to users of our
    // library.
    let mut expensive_result = Cacher::new(&|num: &u32| {
        println!("calculating slowly...");
        // thread::sleep(Duration::from_secs(2));
        *num
    });

    if intensity < 25 {
        println!("Today, do {} pushups!", expensive_result.value(intensity));
        println!("Next, do {} situps!", expensive_result.value(intensity));
    } else if random_number == 3 {
        println!("Take a break today! Remember to stay hydrated!");
    } else {
        println!(
            "Today, run for {} minutes!",
            expensive_result.value(intensity)
        );
    }
}

// All closures implement at least one of the traits: Fn, FnMut, or FnOnce
struct Cacher<'a, K, V>
where
    K: Eq + Hash,
{
    calculation: &'a dyn Fn(&K) -> V,
    cache: HashMap<K, V>,
}

impl<'a, K, V> Cacher<'a, K, V>
where
    K: Eq + Hash,
{
    fn new(calculation: &'a dyn Fn(&K) -> V) -> Cacher<'a, K, V> {
        Cacher {
            calculation,
            cache: HashMap::new(),
        }
    }

    fn value(&mut self, arg: K) -> &V {
        let calculation = self.calculation;
        self.cache.entry(arg).or_insert_with_key(calculation)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn call_with_different_values() {
        let mut c = Cacher::new(&|a: &u32| *a);

        let v1 = c.value(1);
        assert_eq!(v1, &1);

        let v2 = c.value(2);
        assert_eq!(v2, &2);
    }

    #[test]
    fn call_with_strings() {
        let mut c = Cacher::new(&|s: &String| s.chars().count());

        let v1 = c.value(String::from("hello"));
        assert_eq!(v1, &5);

        let v2 = c.value(String::from("abc"));
        assert_eq!(v2, &3);
    }
}
