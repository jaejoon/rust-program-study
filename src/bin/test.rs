
fn test_string (param: &String) {
    println!("text is {}", param);
}

fn test_str (param: &str) {
    println!("text(str) is {}", param);
}

fn test_borrow (param: &str) {
    println!("text(borrow) is {}", param);
}

fn main () {
    let text: String = String::from("Hello");
    let text_str = text.as_str(); //String to String slice
    println!("main: {}", text);
    test_borrow(text_str); //send string slice
    test_string(&text);
    test_str(&text);
}