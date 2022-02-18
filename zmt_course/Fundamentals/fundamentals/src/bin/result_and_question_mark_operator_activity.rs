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
    Mannager,
    LineSupervisor,
    Kitchen,
    Assembly,
}

// * Ensure that terminated employees cannot access the building
//   regardless of their position
enum ContractStatus {
    Active,
    Terminated
}

// * Use a struct to store the employee type and whether they are
//   still employed
struct Employee {
    position: Position,
    status: ContractStatus,
}

// * Use a function that returns a Result to determine if the employee
//   may enter the building
fn check_access_permission(employee: &Employee) -> Result<(), String> {
    match employee.status {
        ContractStatus::Terminated => return Err("Terminated employed can not Access".to_owned()),
        _ => (),
    }

    match employee.position {
        Position::Maintenance => Ok(()),
        Position::Mannager => Ok(()),
        Position::Marketing => Ok(()),
        _ => Err("Access Denied".to_owned()),
    }
    //Ok(())
}

//   * Must use a function that utilizes the question mark operator to do this
fn print_access_status(employee: &Employee) -> Result<(), String> {
     check_access_permission(employee)?;
    // * Print whether the employee may access the building
    println!("Allow Access");
    Ok((),)
}


fn main() {

    let employees = vec![
        Employee {
            position: Position::Maintenance,
            status: ContractStatus::Active      
        },
        Employee {
            position: Position::Marketing,
            status: ContractStatus::Terminated      
        },
        Employee {
            position: Position::Mannager,
            status: ContractStatus::Active      
        },
        Employee {
            position: Position::LineSupervisor,
            status: ContractStatus::Active  
        },
        Employee {
            position: Position::Kitchen,
            status: ContractStatus::Active
        },
        Employee {
            position: Position::Assembly,
            status: ContractStatus::Terminated
        },
    ];

    for emp in employees {        
        match print_access_status(&emp) {
            Err(e) => println!("{:?}", e),
            _ => (),
        }
    }
}