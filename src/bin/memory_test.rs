#[derive(Debug)]
enum Test {
    Yes,
    No,
}

fn main() {
    // variable yes is created in Stack
    let yes = Test::Yes;
    println!("yes: {:?}", yes);

    // Box::new(yes) put "yes" in the heap and return heap address, so yes_heap is to be address of heap memory
    let yes_heap: Box<Test> = Box::new(yes);
    println!("yes_heap: {:p}", yes_heap);

    // *yes_heap is the value of pointed by yes_heap. yes_heap is heap address. *yes_heap is the value its pointer
    let yes_stack = *yes_heap;
    println!("yes_stack: {:?}", yes_stack);
}