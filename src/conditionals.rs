// conditionals - used to check the conditional of something 

pub fn run() {
    let age: u8 = 18;
    let check_id: bool = false;
    let knows_person_is_of_age = true;

    if knows_person_is_of_age || (age >= 21 && check_id) {
        println!("bartender: sure things!");
    } else if age < 21 && check_id {
        println!("uh no");
    }

    // shorthand if 
    let is_of_age = if age > 21 { true } else { false };
    println!("is of age: {}", is_of_age);
}