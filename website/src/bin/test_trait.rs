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

pub trait General {
    fn get_name(&self) -> String;
}

fn demo<T: General>(people: T) {
    println!("Hi! Your name is: {}", people.get_name());
}

pub struct D1 {
    name: String,
}

impl General for D1 {
    fn get_name(&self) -> String {
        format!("{}", self.name)
    }
}

fn main() {
    let peti = D1 {
        name: "Peti".to_string(),
    };

    println!("Name is: {}", peti.get_name());
    demo(peti);
}
