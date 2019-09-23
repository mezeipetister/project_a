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
