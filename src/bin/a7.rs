// Topic: Working with an enum
//
// Program requirements:
// * Prints the name of a color to the terminal
//
// Notes:
// * Use an enum with color names as variants
// * Use a function to print the color name
// * The function must use the enum as a parameter
// * Use a match expression to determine which color
//   name to print

enum Colors {
    White,
    Black,
    Yellow,
    Pink
}

fn display_color(color: Colors) {
    match color {
        Colors::Black => println!("black"),
        Colors::Pink => println!("pink"),
        Colors::White => println!("white"),
        Colors::Yellow => println!("yellow"),
    }
}
fn main() {
    let my_color = Colors::Black;
    display_color(my_color);
}
