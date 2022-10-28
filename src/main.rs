fn add(x: i32, y: i32) -> i32 {
    x + y
}

fn main() {
    println!("Hello, world!");
    let mut x: i32 = 32;
    //let x: i32 = 32;
    let y: i32 = 5;
    let z: f64 = 2.0;
    let a: String = String::from("Hello");
    let b: String = "World".to_string();
    let sum: i32 = x + y;
    println!("Sum of {} and {} is {} 😀", x, y, sum);
    println!("{}", z);
    println!("{}", a);
    println!("{}", b);
    x += 2;
    let w: i32 = add(x, y);
    println!("{}", x);
    println!("{}", w);
}
