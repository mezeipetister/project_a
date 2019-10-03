extern crate crypto;
extern crate jwt as jsonwebtoken;
#[macro_use]
extern crate serde_derive;

pub mod jwt;

pub fn demo_function() -> String {
    "HelloWorld".to_string()
}
