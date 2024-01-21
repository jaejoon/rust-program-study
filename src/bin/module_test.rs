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

mod math {
    pub fn add(a: i32, b: i32) -> i32 {
        a+b
    }
    
    pub fn sub(a: i32, b: i32) -> i32 {
        a-b
    }
}

fn main() {
    use greet::*;
    use math::*;
    
    hello();
    goodbye();
    add(1,2);
    sub(3,1);
}