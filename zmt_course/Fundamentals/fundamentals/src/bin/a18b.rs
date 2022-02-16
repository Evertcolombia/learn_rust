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



// * Use an enum to represent all types of employees
enum Position {
    Maintenance,
    Marketing,
    Managers,
    Line,
    Kitchen,
    Assembly,
}

enum Status {
    Active,
    Terminated,
}
// * Use a struct to store the employee type and whether they are
// still employed
struct Employee {
    position: Position,
    status: Status,
}


// * Use a function that returns a Result to determine if the employee
//   may enter the building
fn try_access(employee: &Employee) -> Result<(), String> {
    match employee.status {
        Status::Terminated => return Err("Terminated contract".to_owned()),
        _ => (),
    }

    match employee.position {
        Position::Maintenance => Ok(()),
        Position::Marketing => Ok(()),
        Position::Managers => Ok(()),
        _ => Err("Empleoyee position has not access allowed".to_owned()),
    }
}

fn print_access(employee: &Employee) -> Result<(), String> {
    let access_attempt = try_access(employee)?;
    println!("Access ok");
    Ok(())
}



fn main() {
    let manager = Employee {
        position: Position::Managers,
        status: Status::Active,
    };

    let res = print_access(&manager);

    match res {
        Err(e) => println!("Access Denied: {:?}", e),
        _ => (),
    }

}