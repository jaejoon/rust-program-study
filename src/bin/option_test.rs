
/// Survey structure
struct Survey {
    q1: Option<i32>,
    q2: Option<bool>,
    q3: Option<String>,
    q4: Option<i64>,
}

/// main function for printing survey responses
fn main () {
    let response: Survey = Survey { 
        q1: Some(12), 
        q2: Some(true), 
        q3: Some("A".to_owned()), 
        q4: None,
    };

    match response.q1 {
        Some(ans) => println!("q1 ans: {:?}", ans),
        None => println!("q1 none"),
    }
    match response.q2 {
        Some(ans) => println!("q2 ans: {:?}", ans),
        None => println!("q2 none"),
    }
    match response.q3 {
        Some(ans) => println!("q3 ans: {:?}", ans),
        None => println!("q3 none"),
    }
    match response.q4 {
        Some(ans) => println!("q4 ans: {:?}", ans),
        None => println!("q4 none"),
    }
}