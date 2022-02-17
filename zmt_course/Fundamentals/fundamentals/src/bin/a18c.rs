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
#[derive(Debug)]
enum Position {
    Maintenance,
    Marketing,
    Mannager,
    LineSupervisor,
    KitchenStaff,
    Aseembly,
}

// * enum to represent the status of an employee
#[derive(Debug)]
enum Status {
    Active,
    Terminated,
}

// * Use a struct to store the employee type and whether they are
//   still employed
#[derive(Debug)]
struct Employee {
    position: Position,
    status: Status
}

// * Use a function that returns a Result to determine if the employee
//   may enter the building
fn check_employee_access(employee: &Employee) -> Result<(), String> {
    
    match employee.status {
        Status::Terminated => return Err("Yet this person does not work for the company".to_owned()),
        _ => (),
    }

    match employee.position {
        Position::Marketing => Ok(()),
        Position::Mannager => Ok(()),
        Position::Maintenance => Ok(()),
        _ => Err("Empleoyee position has not access allowed".to_owned()),
    }
}

// * Print whether the employee may access the building
//   * Must use a function that utilizes the question mark operator to do this
fn print_access_status(employee: &Employee) -> Result<(), String> {
    let access_attemp = check_employee_access(employee)?;
    println!("Accesss Guaranted");
    Ok(())
}

fn main() {
    let employee = Employee {
        position: Position::Maintenance,
        status: Status::Active
    };

    
    //let res = check_employee_access(&employee);
    //let res = print_access_status(&employee);
    match print_access_status(&employee) {
        Err(e) => println!("Access Denied: {:?}", e),
        _ => (),
    }
    
}