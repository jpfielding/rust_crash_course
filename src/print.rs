pub fn run() {
    // print to console
    println!("hello from the print.rs file");

    // positional args
    println!(
        "{} is from {} and {0} likes to {2}", 
        "JP", "NoVA", "code"
    );

    // named args
    println!(
        "{name} likes to play {activity}",
        name = "jp",
        activity = "lacrosse"
    );

    // placeholder traits
    println!("binary: {:b} Hex: {:x} Octal: {:o}", 10, 10, 10);
}