fn maybe_num () -> Option<i32> {
    Some(1)
}

fn maybe_word () -> Option<String> {
    Some("Hello".to_owned())
}

fn main () {
    // let plus_one = match maybe_num() {
    //     Some(num) => Some(num+1),
    //     None => None,
    // };

    // make short above code using map combinator .map(F)
    let plus_one = maybe_num().map(|num| num+1);
    println!("num: {:?}", plus_one);

    // match plus_one {
    //     Some(num) => println!("match num: {}", num),
    //     None => (),
    // }
    
    // map chain
    let word_length = maybe_word()
        .map(|word| word.len())
        .map(|len| len*2);
    println!("word length: {:?}", word_length);

}