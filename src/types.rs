/*
primitive types---
ints: u/i,  8-128
floats: f32, f64
boolean: bool
chars: char
tuples 
arrays
*/

pub fn run(){
    let x = 1;      // i32 by default
    let y = 2.5;    // f64 by default
    let z: i64 = 4545445454545;
    // find max size
    println!("max i32: {}", std::i32::MAX);
    println!("max i64: {}", std::i64::MAX);

    let is_active: bool = true;

    let is_greater: bool = 10 < 5;

    let a1 = 'a';

    println!("{:?}", (x, y, z, is_active, is_greater, a1));
}