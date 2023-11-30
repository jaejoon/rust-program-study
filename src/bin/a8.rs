// Topic: Organizing similar data using structs
//
// Requirements:
// * Print the flavor of a drink and it's fluid ounces
//
// Notes:
// * Use an enum to create different flavors of drinks
// * Use a struct to store drink flavor and fluid ounce information
// * Use a function to print out the drink flavor and ounces
// * Use a match expression to print the drink flavor

// * Use an enum to create different flavors of drinks
enum Flavors {
    Sweet,
    Sour,
    Bitter,
}

struct Drinks {
    drink_flavor: Flavors,
    ounce: f64,
}

fn print_drink_info(drink: Drinks) {
    match drink.drink_flavor {
        Flavors::Sweet => println!("Sweet"),
        Flavors::Bitter => println!("Bitter"),
        Flavors::Sour => println!("Sour"),
    }
    println!("ounce: {}", drink.ounce);
}

fn main() {
    let sweet = Drinks {
        drink_flavor: Flavors::Sweet,
        ounce: 3.99,
    };
    print_drink_info(sweet);

    let bitter = Drinks {
        drink_flavor: Flavors::Bitter,
        ounce: 4.99,
    };
    print_drink_info(bitter);

    let sour = Drinks {
        drink_flavor: Flavors::Sour,
        ounce: 1.99,
    };
    print_drink_info(sour);

}
