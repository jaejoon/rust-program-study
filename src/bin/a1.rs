// Topic: Functions
//
// Program requirements:
// * Displays your first and last name
//
// Notes:
// * Use a function to display your first name
// * Use a function to display your last name
// * Use the println macro to display messages to the terminal

fn first_name(first_name: &str) -> &str {
    first_name
}

fn last_name(last_name: &str) -> &str {
    last_name
}

fn display_name() {
    let first_name = first_name("Jaejoon");
    let last_name = last_name("Jung");
    println!("{} {}",first_name, last_name);
}
fn main() {
    display_name();
}
