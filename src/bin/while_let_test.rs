fn main() {
    let mut data = Some(3);

    while let Some(i) = data {
        println!("while loop"); 
        data = None;       
    }

    let numbers = vec![1,2,3,4,5];
    let mut numbers_iter = numbers.iter();

    while let Some(num) = numbers_iter.next() {
        println!("number: {:?}", num);
    }
    println!("done");
}