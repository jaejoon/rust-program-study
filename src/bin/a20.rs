// Topic: User input
//
// Requirements:
// * Verify user input against pre-defined keywords
// * The keywords represent possible power options for a computer:
//   * Off
//   * Sleep
//   * Reboot
//   * Shutdown
//   * Hibernate
// * If the user enters one of the keywords, a message should be printed to
//   the console indicating which action will be taken
//   * Example: If the user types in "shutdown" a message should display such
//     as "shutting down"
// * If the keyword entered does not exist, an appropriate error message
//   should be displayed
//
// Notes:
// * Use an enum to store the possible power states
// * Use a function with a match expression to print out the power messages
//   * The function should accept the enum as an input
// * Use a match expression to convert the user input into the power state enum
// * The program should be case-insensitive (the user should be able to type
//   Reboot, reboot, REBOOT, etc.)
use std::io;

enum PowerStates {
    Off,
    Sleep,
    Reboot,
    Shutdown,
    Hibernate,
}

impl PowerStates {
    fn new (param: &str) -> Option<PowerStates> {
        let param: String = param.trim().to_lowercase();
        // String to str with .as_str()
        match param.as_str() {
            "off" => Some(PowerStates::Off),
            "sleep" => Some(PowerStates::Sleep),
            "reboot" => Some(PowerStates::Reboot),
            "shutdown" => Some(PowerStates::Shutdown),
            "hibernate" => Some(PowerStates::Hibernate),
            _ => None,
        }
    }

}

fn print_power_action (input: PowerStates) {
    // can remove "PowerState::" in match
    use PowerStates::*;
    match input {
        Off => println!("Power off"),
        Reboot => println!("System reboot"),
        Shutdown => println!("System shutdown"),
        Sleep => println!("System sleep"),
        Hibernate => println!("System hibernate"),
    }
    // match input {
    //     PowerStates::Off => println!("Power off"),
    //     PowerStates::Reboot => println!("System reboot"),
    //     PowerStates::Shutdown => println!("System shutdown"),
    //     PowerStates::Sleep => println!("System sleep"),
    //     PowerStates::Hibernate => println!("System hibernate"),
    // }
}

fn main() {
    let mut buffer = String::new();
    println!("Enter new power state: ");
    let user_input = io::stdin().read_line(&mut buffer);
    if user_input.is_ok() {
        match PowerStates::new(&buffer) {
            Some(state) => print_power_action(state),
            None => println!("Invaild power state"),
        }
    } else {
        println!("Error reading input");
    }
}
