// vectors - are resizable arrays

use std::mem;

pub fn run() {
    let mut numbers: Vec<i32> = vec![1, 2, 3, 4];

    // re-assign value
    numbers[2] = 20;

    // vectors can re-size
    numbers.push(5);
    numbers.push(6);

    // and we can remove
    numbers.pop();

    println!("{:?}", numbers);

    // get single va
    println!("{}", numbers[0]);

    // get array length
    println!("vector length: {}", numbers.len());

    // arrays are stack allocated
    println!("vector occupies {} bytes", mem::size_of_val(&numbers));

    // get slice
    let slice: &[i32] = &numbers[0..2];
    println!("slice: {:?}", slice);

    // loop through vector values
    for x in numbers.iter() {
        println!("Number: {}", x);
    }

    // loop & mutate
    for x in numbers.iter_mut() {
        *x *= 2;
    }

    println!("{:?}", numbers);
}