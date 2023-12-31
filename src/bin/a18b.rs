// Topic: Result & the question mark operator
//
// Requirements:
// * Determine if an employee can access a building using a digital keycard
// * Employees that can access the building are:
//   * Maintenance crews
//   * Marketing department employees
//   * Managers
// * Other employees that work at the company are:
//   * Line supervisors
//   * Kitchen staff
//   * Assembly technicians
// * Ensure that terminated employees cannot access the building
//   regardless of their position
//
// Notes:
// * Use an enum to represent all types of employees
// * Use a struct to store the employee type and whether they are
//   still employed
// * Use a function that returns a Result to determine if the employee
//   may enter the building
// * Print whether the employee may access the building
//   * Must use a function that utilizes the question mark operator to do this

enum Position {
    Maintenance,
    Marketing,
    Manager,
    LineSupervisor,
    KitchenStaff,
    AssemblyTech,
}

enum Status {
    Active,
    Terminated,
}
struct Employees {
    position: Position,
    status: Status,
}

fn check_access(emp: &Employees) -> Result<(), String> {
    match emp.status {
        Status::Terminated => return Err("Terminated employee".to_owned()),
        _ => (), 
    }

    match emp.position {
        Position::Maintenance => Ok(()),
        Position::Marketing => Ok(()),
        Position::Manager => Ok(()),
        _ => Err("Invaild position".to_owned()), 
    }
}

fn print_access(emp: &Employees) -> Result<(), String> {
    let _check_access_result = check_access(emp)?;
    println!("access ok");
    Ok(())
}

fn main() {
    let maintenance = Employees {
        position: Position::Maintenance,
        status: Status::Terminated,
    };

    match print_access(&maintenance) {
        Err(e) => println!("access denied {:?}", e),
        _ => (),
    }
}
