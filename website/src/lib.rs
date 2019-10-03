// Copyright (C) 2019 Peter Mezei
//
// This file is part of Project A.
//
// Project A is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 2 of the License, or
// (at your option) any later version.
//
// Project A is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with Project A.  If not, see <http://www.gnu.org/licenses/>.

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
