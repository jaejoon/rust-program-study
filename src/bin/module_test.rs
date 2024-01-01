use std::collections::HashMap; // this can be used in main function

mod greet {
    use std::collections::HashMap; // you can use this Hashpmap in greet module

    pub fn hello() { // "pub" is needed use in main. "pub" means public function 
        println!("Hello");
    }
    
    pub fn goodbye() {
        println!("Goodbye");
    }
}

pub mod math {
    fn add(a: i32, b: i32) -> i32 {
        a+b
    }
    
    fn sub(a: i32, b: i32) -> i32 {
        a-b
    }
}

fn main() {
    use greet::*;
    hello();
    goodbye();
}