/// primitve str = immutable fixed length string somwerhe in memory
// string = growable, heap-allocated data structure = user when you need to modify or own

pub fn run() {
    let mut hello = String::from("Hello");

    // get length

    println!("length: {}", hello.len());

    hello.push('W');
    hello.push_str("orld!");

    // capacity in bytes
    println!("capacity: {}", hello.capacity());
    println!("isempty: {}", hello.is_empty());
    println!("contains 'World': {}", hello.contains("World"));
    println!("replace: {}", hello.replace("World", "There"));

    // loop through string by whitespace
    for word in hello.split_whitespace() {
        println!("{}", word);
    }

    let mut s = String::with_capacity(10);
    s.push('a');
    s.push('b');

    // assertion testing
    assert_eq!(2, s.len());
    assert_eq!(10, s.capacity());

    println!("{}", hello);
}