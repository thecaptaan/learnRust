// Variables
// All variables in Rust are immutable by default.
// You need to use mut keyword to make them mutable.
// Rust is a statically typed language, which means that it must know the types of all variables at compile time.
// The compiler can usually infer what type we want to use based on the value and how we use it.

fn main() {
    // Integer
    let a: i32 = 10;
    let b: i64 = 1000000000000000000;
    let c: u32 = 100;
    println!("a: {}, b: {}, c: {}", a, b, c);

    // Float
    let d: f32 = 10.0;
    let e: f64 = 100.0;
    println!("d: {}, e: {}", d, e);

    // Boolean
    let f: bool = true;
    let g: bool = false;
    println!("f: {}, g: {}", f, g);

    // Character
    let h: char = 'A';
    println!("h: {}", h);

    // String
    let i: &str = "Hello, World!";
    println!("i: {}", i);

    // Mutable variable
    let mut j: i32 = 10;
    j = 20;
    println!("j: {}", j);
}
