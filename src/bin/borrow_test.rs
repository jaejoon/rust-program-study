
fn foo1 (param: String) {
    println!("foo print param(val1): {:?}", param);
}

fn foo2 (param: &String) {
    println!("foo print param(val2): {:?}", param);
}
fn main () {
    // main has val ownership
    let val1 = String::from("Borrow test val1");
    foo1(val1);
    // val을 사용할 수 없음. 왜냐하면 foo에 소유권을 넘겼고 foo에서 val를 삭제했기 때문.
    // println!("main print val: {:?}", val); // compile error

    let val2: String = String::from("Borrow test val2");
    foo2(&val2);
    println!("main print val2: {:?}", val2); // compile Ok because of borrowed val2's ownership into foo2

}