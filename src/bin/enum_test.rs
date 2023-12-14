
#[derive(Debug)]
enum Test {
    Red(i32), // this variant has additional data typed i32
    Blue(i32),
    Green(i32),
    Mixed(Complex), // this variant has additional data typed as structure    
}

#[derive(Debug)]
struct  Complex {
    red: i32,
    blue: i32,
    green: i32,
}

fn main () {
    let complex = Complex {
        red: 10,
        blue: 12,
        green: 100,
    };
    println!("red, blue, green: {:}, {:}, {:}", complex.red,complex.blue,complex.green);
    let mixed_light: Test = Test::Mixed(complex);
    let red_light: Test = Test::Red(1000);
    let blue_light: Test = Test::Blue(2000);
    let green_light: Test = Test::Green(3000);

    // borrow of move at "let mixed_light: Test = Test::Mixed(complex);" line
    // println!("red, blue, green: {:}, {:}, {:}", complex.red,complex.blue,complex.green);
    println!("mixed: {:?}", mixed_light);
    println!("red: {:?}", red_light);
    println!("blue: {:?}", blue_light);
    println!("green: {:?}", green_light);
}