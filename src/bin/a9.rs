// Topic: Data management using tuples
//
// Requirements:
// * Print whether the y-value of a cartesian coordinate is
//   greater than 5, less than 5, or equal to 5
//
// Notes:
// * Use a function that returns a tuple
// * Destructure the return value into two variables
// * Use an if..else if..else block to determine what to print

fn coord () -> (i32, i32) {
    (10,7)
}

fn main() {
    let (x, y) = coord();

    if y > 5 {
        println!("Y is greater than 5");
    } else if y < 5 {
        println!("Y is less than 5");
    } else {
        println!("Y is equal to 5");
    }

    println!("and X is {}", x)
}
