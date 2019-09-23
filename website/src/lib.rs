extern crate core_lib;

pub struct Dog {
    name: String,
    age: i32,
}

impl Dog {
    fn new() -> Dog {
        Dog {
            name: "".to_string(),
            age: 0,
        }
    }
}

pub trait Animal {
    fn say_hello(&self) -> String;
}

impl Animal for Dog {
    fn say_hello(&self) -> String {
        String::from("Hello")
    }
}

fn demo<T: Animal>(x: T) {
    x.say_hello();
    core_lib::demo_function();
}
