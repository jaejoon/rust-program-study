// Topic: HashMap
//
// Requirements:
// * Print the name and number of items in stock for a furniture store
// * If the number of items is 0, print "out of stock" instead of 0
// * The store has:
//   * 5 Chairs
//   * 3 Beds
//   * 2 Tables
//   * 0 Couches
// * Print the total number of items in stock
//
// Notes:
// * Use a HashMap for the furniture store stock
use std::collections::HashMap;

#[derive(Debug)]
struct Furnitures {
    name: String,
    count: i32,
}

fn main() {
    let mut stocks = HashMap::new();
    stocks.insert(
        1,
        Furnitures {
            name: "chair".to_owned(),
            count: 5,
        },
    );
    stocks.insert(
        2,
        Furnitures {
            name: "bed".to_owned(),
            count: 3,
        },
    );
    stocks.insert(
        3,
        Furnitures {
            name: "table".to_owned(),
            count: 2,
        },
    );
    stocks.insert(
        4,
        Furnitures {
            name: "couche".to_owned(),
            count: 0,
        },
    );

    let mut total_count = 0;
    for (stock_number, furniture) in stocks.iter() {
        if furniture.count > 0 {
            println!("stock number: {:?}, Funiture name: {:?}, count: {:?}", stock_number, furniture.name, furniture.count);
            total_count = total_count + furniture.count;
        }else {
            println!("stock number: {:?}, Funiture name: {:?} out of stock", stock_number, furniture.name);    
        }
    }
    println!("Total count is {:?}", total_count);
}
