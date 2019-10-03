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

fn double(num: i32) -> i32 {
    num * 2
}

fn main() {
    let x: i32 = 5;
    let y = &x;
    let z = &x as *const i32;

    println!("x => {}, y => {}", x, y);
    println!("x => {}, *y => {}", x, *y);
    println!("x => {}, &y => {}", x, &y);
    println!("Z => {:?}", z);
    println!("y as raw pointer => {:?}", y as *const i32);

    println!("Result: {}", double(*y));
}
