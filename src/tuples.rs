// tuples group together values of diff types
// max 12 elems
pub fn run() {
    let person: (&str, &str, i8) = ("jp", "NoVA", 45);

    println!("{} is from {}, and is {}", person.0, person.1, person.2);
}