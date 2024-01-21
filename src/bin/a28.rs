// Topic: New type pattern
//
// Requirements:
// * Display the selected color of shoes, a shirt, and pants
// * Create and display at least one of each type of clothes and color
//
// Notes:
// * Create a new type for each clothing item that wraps the Color enum
//   * Each new type should implement a `new` function
// * Create a function for each type of clothes (shoes, shirt, pants)
//   that accepts the new type specific to that type of clothing

#[derive(Debug)]
enum Color {
    Black,
    Blue,
    Brown,
    Custom(String),
    Gray,
    Green,
    Purple,
    Red,
    White,
    Yellow,
}

#[derive(Debug)]
struct ShirtColor(Color);
impl ShirtColor {
    fn new(color: Color) -> Self {
        Self(color)
    }
}

#[derive(Debug)]
struct ShoeColor(Color);
impl ShoeColor {
    fn new(color: Color) -> Self {
        Self(color)
    }
}

#[derive(Debug)]
struct PantsColor(Color);
impl PantsColor {
    fn new(color: Color) -> Self {
        Self(color)
    }
}

fn print_shirts_color(color: ShirtColor) {
    println!("shirts color {:?}", color);
}
fn print_shoe_color(color: ShoeColor) {
    println!("shoe color {:?}", color);
}
fn print_pants_color(color: PantsColor) {
    println!("pants color {:?}", color);
}

fn main() {
    let shirt_color  = ShirtColor::new(Color::Blue);
    let pants_color = PantsColor::new(Color::Black);
    let shoe_color = ShoeColor::new(Color::Red);

    print_shirts_color(shirt_color);
    print_pants_color(pants_color);
    print_shoe_color(shoe_color);
}
