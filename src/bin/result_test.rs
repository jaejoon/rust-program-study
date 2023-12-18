#[derive(Debug)]
enum MenuChoice {
    MainMenu,
    Start,
    Quit,
}

fn get_choice (input: &str) -> Result<MenuChoice, String> {
    match input {
        "mainmenu" => Ok(MenuChoice::MainMenu),
        "start" => Ok(MenuChoice::Start),
        "quit" => Ok(MenuChoice::Quit),
        _ => Err("Unknown menu".to_owned()),
    }
}

fn print_choice (choice: &MenuChoice) {
    println!("choice: {:?}", choice);
}

fn pick_choice (input: &str) -> Result<(), String> {
    let choice = get_choice(input)?;
    print_choice(&choice);
    Ok(())
}
fn main () {
    // Ok case
    let choice = get_choice("mainmenu");
    // Err case
    // let choice = get_choice("leave");

    match choice {
        Ok(inner_choice) => print_choice(&inner_choice),
        Err(e) => println!("error: {:?}", e),
    }

    // another simple way using ?
    let second_choice = pick_choice("start");
    println!("second choice: {:?}", second_choice);

    let third_choice = pick_choice("left");
    println!("third choice: {:?}", third_choice);
}