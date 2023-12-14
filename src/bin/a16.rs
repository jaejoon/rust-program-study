// Topic: Option
//
// Requirements:
// * Print out the details of a student's locker assignment
// * Lockers use numbers and are optional for students
//
// Notes:
// * Use a struct containing the student's name and locker assignment
// * The locker assignment should use an Option<i32>

// * Use a struct containing the student's name and locker assignment
// * The locker assignment should use an Option<i32>
struct Lockers {
    name: String,
    locker_num: Option<i32>,
}
fn main() {
    let lockers = vec![
        Lockers {
            name: "Joe".to_owned(),
            locker_num: Some(1),
        },
        Lockers {
            name: "Matthew".to_owned(),
            locker_num: None,
        },
        Lockers {
            name: "Jin".to_owned(),
            locker_num: Some(10),
        },
    ];

    for locker in lockers {
        // use match
        match locker.locker_num {
            Some(num) => println!("Name: {:?}, Assigned Locker number: {:?}", locker.name, num),
            None => println!("No locker Assigned to {:?}", locker.name),
        }

        // use if 
        if locker.locker_num != None {
            println!("Name: {:?}, Assigned Locker number: {:?}", locker.name, locker.locker_num);
        } else {
            println!("No locker Assigned to {:?}", locker.name);
        }
    }
}
