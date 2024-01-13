fn main() {
    let numbers = vec![1,2,3,4,5];

    // let mut plus_one = vec![];
    // for num in numbers {
    //     plus_one.push(num+1);
    // }
    
    // change above for loop with iterator
    let plus_one: Vec<_> = numbers
        .iter()
        .map(|num| num+1)
        .collect(); // collect returns vector
    dbg!(plus_one);

    let numbers = vec![1,2,3,4,5];
    let filtered_number: Vec<_> = numbers
        .iter()
        .filter(|num| num <= &&3)
        .collect();
    dbg!(filtered_number);

    let numbers = vec![1,2,3,4,5];
    let find_number = numbers
        .iter()
        .find(|num| num == &&40);
    dbg!(find_number);

    let count = numbers
        .iter()
        .count();
    dbg!(count);

    let last = numbers
        .iter()
        .last();
    dbg!(last);

    let numbers = vec![1,2,3,4,5];
    let min = numbers
        .iter()
        .min();
    dbg!(min);

    let max = numbers
        .iter()
        .max();
    dbg!(max);

    let take: Vec<_> = numbers
        .iter()
        .take(3)
        .collect();
    dbg!(take);
}