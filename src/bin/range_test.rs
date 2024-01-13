fn main() {
    let range = 1..=3;
    let range1 = 1..4;

    for num in 1..5 {
        println!("{:?}", num);
    }

    for num in 1..=5 {
        println!("{:?}", num);
    }

    for char in 'a'..='d' {
        println!("{:?}", char);
    }
}