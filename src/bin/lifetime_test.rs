#[derive(Debug)]
enum Answer {
    Yes,
    No,
}

#[derive(Debug)]
struct Form<'a> {
    question: &'a Answer,
}

#[derive(Debug)]
struct Quize {
    question: Answer,
}

fn get_first_question<'a>(quiz_1: &'a Quize, quiz_2: &Quize) -> &'a Answer {
    &quiz_1.question
}
fn main() {
    let answer = Answer::Yes;

    let form = Form {
        question: &answer
    };

    println!("{:?}", form);
}