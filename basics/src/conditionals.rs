pub fn run() {
    let age = 18;
    let check_id: bool = true;
    let knows_person_of_age = true;

    // If/Else
    if age >= 21 && check_id  || knows_person_of_age {
        println!("Bartender: What would you like to drink?");
    } else if age >= 18 {
        println!("Bartender: What's your favorite juice?");
    } else {
        println!("Bartender: Sorry, you have to leave");
    }

    // Shorthand If 
    let is_of_age = if age >= 21 { true } else { false };
    println!("Is Of Age: {}", is_of_age);
}