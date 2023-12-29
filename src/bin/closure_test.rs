
fn add_fn (a: i32, b: i32) -> i32 {
    a+b
}

fn main () {
    //closure form 
    let add = |a: i32, b:i32| -> i32 {
        a+b
    };
    // closure short form. 
    // type이 필요하지 않는 이유는 아래 sum에서 클로저 add를 호출할때 넣는 a,b의 값으로 컴파일러가 type을 추정하기 때무임. 
    let add = |a, b| a+b;
    let sum = add(10, 30);
    println!("sum: {:?}", sum);
    
    let sum_fn = add_fn(10,10);
    println!("sum_fn: {:?}", sum_fn);
}