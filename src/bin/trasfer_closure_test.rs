fn math(a: f32, b: f32, op: Box<dyn Fn(f32, f32) -> f32>) -> f32 {
    op(a, b)
}

fn main() {
    let add: Box<_> = Box::new( 
        |a, b| {
            println!("add ");
            a+b
        });

    let sub: Box<_> = Box::new( 
        |a, b| {
            println!("sub ");
            if a > b {
                a-b
            } else {
                b-a
            }
        });
    let mul: Box<_> = Box::new(
        |a, b| {
            println!("mul ");
            a*b
        });

    let div: Box<_> = Box::new(
        |a, b| {
            println!("div ");
            if b != 0.0 {
                a / b
            } else {
                println!("Error! divided by zero. b is zero");
                // set default return value MUST!!
                0.0
            }
        });
    println!("{}", math(2.,3., add));
    println!("{}", math(2.,3., sub));
    println!("{}", math(2.,3., mul));
    println!("{}", math(2.,3., div));

}