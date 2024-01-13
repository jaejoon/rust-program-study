enum Colors {
    red,
    blue,
    green,
}
fn main() {
    let maybe_user = Some("Jarry");
    match maybe_user {
        Some(user) => println!("User name: {:?}", user),
        None => println!("No user")
    }

    if let Some(user) = maybe_user {
        println!("User name: {:?}", user);
    } else {
        println!("No user");
    }

    let color = Colors::red;
    if let Colors::red = color {
        println!("It's red");
    } else {
        println!("Not red");
    }
}