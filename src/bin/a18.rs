// Topic: Result
//
// Requirements:
// * Determine if a customer is able to make a restricted purchase
// * Restricted purchases require that the age of the customer
//   is at least 21
//
// Notes:
// * Use a struct to store at least the age of a customer
// * Use a function to determine if a customer can make a restricted purchase
// * Return a result from the function
// * The Err variant should detail the reason why they cannot make a purchase

struct Customer {
    age: i32,
}

fn try_purchase (customer: &Customer) -> Result<(), String> {
    if customer.age >= 21 {
        println!("Available age, Customers age is {:?}", customer.age);
        Ok(())
    } else {
        //println!("Not available age: {:?}", customer.age);
        Err("Not available purchase. Customer must be at least 21 years old".to_owned())
    }

}
fn main() {
    let james = Customer {
        age: 20,
    };

    let purchased = try_purchase(&james);
    println!("{:?}", purchased);
}
