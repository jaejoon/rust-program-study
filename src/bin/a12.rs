// Topic: Implementing functionality with the impl keyword
//
// Requirements:
// * Print the characteristics of a shipping box
// * Must include dimensions, weight, and color
//
// Notes:
// * Use a struct to encapsulate the box characteristics
// * Use an enum for the box color
// * Implement functionality on the box struct to create a new box
// * Implement functionality on the box struct to print the characteristics
enum BoxColor {
    red,
    white,
    black,
}
impl BoxColor {
    fn print(&self) {
        match self {
            BoxColor::black => println!("Black"),
            BoxColor::white => println!("White"),
            BoxColor::red => println!("Red"),
        }
    }
}
struct Dimension {
    height: f64,
    width: f64,
    depth: f64,
}
impl Dimension {
    fn print(&self) {
        println!("hight: {:?}, width: {:?}, depth: {:?}", self.height, self.width, self.depth);
    }
}
struct ShippingBox {
    color: BoxColor,
    weight: f64,
    dimension: Dimension,
}

impl ShippingBox {
    fn new(weight: f64, color: BoxColor, dimension: Dimension) -> Self {
        Self {
            color,
            weight,
            dimension,
        }
    }
    fn print(&self) {
        self.color.print();
        self.dimension.print();
        println!("weight: {:?}", self.weight);
    }
}

fn main() {
    let small_dimension = Dimension {
        width: 2.3,
        height: 1.1,
        depth: 4.1,
    };
    let small_box = ShippingBox::new(1.3, BoxColor::red, small_dimension);
    small_box.print();
}
