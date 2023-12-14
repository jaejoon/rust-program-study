// Topic: Strings
//
// Requirements:
// * Print out the name and favorite colors of people aged 10 and under
//
// Notes:
// * Use a struct for a persons age, name, and favorite color
// * The color and name should be stored as a String
// * Create and store at least 3 people in a vector
// * Iterate through the vector using a for..in loop
// * Use an if expression to determine which person's info should be printed
// * The name and colors should be printed using a function

// * Use a struct for a persons age, name, and favorite color
// * The color and name should be stored as a String
struct Person {
    age: i32,
    name: String,
    color: String,
}

fn print_namecolor(name: &str, color: &str) {
    println!("name: {:?}", name);
    println!("favorite color: {:?}", color);
}

fn main() {
    let people = vec![
        Person{
            age: 10,
            name: "James".to_owned(),
            color: String::from("Red"),
        },
        Person{
            age: 48,
            name: "Matthew".to_owned(),
            color: String::from("Blue"),
        },
        Person{
            age: 3,
            name: "Jason".to_owned(),
            color: String::from("White"),
        }
    ];

    for info in people {
        if info.age <= 10 {
            print_namecolor(&info.name, &info.color);
        }
    }
}
