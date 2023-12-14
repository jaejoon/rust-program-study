// match_test.rs

enum Discount {
    Percent(i32),
    Flat(i32),
}

struct Ticket {
    event: String,
    price: i32,
}

fn main () {
    let n = 5;
    match n {
        3 => println!("Three"),
        other => println!("other: {:?}", other),
    }

    let flat: Discount = Discount::Flat(10);
    match flat {
        Discount::Flat(2) => println!("flat two"),
        Discount::Flat(other) => println!("other: {:?}", other),
        _ => (),
    }

    let concert = Ticket {
        event: "concert".to_owned(),
        price: 10,
    };
    match concert {
        Ticket {price: 50, event} => println!("event at price 50: {:?}", event),
        Ticket {price, ..} => println!("price: {:?}", price),
    }
}
