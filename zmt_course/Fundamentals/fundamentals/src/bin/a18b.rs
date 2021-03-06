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
enum EmployeePosition {
    Maintenance, 
    Marketing,
    Mannager,
    LineSupervisor,
    KitchenStaff,
    AssemblyTec
}

impl EmployeePosition {
    
}

enum EmployeeStatus {
    Actived,
    Terminated
}



// * Use a struct to store the employee type and whether they are
//   still employed
struct Employee {
    position: EmployeePosition,
    status: EmployeeStatus
}


// * Use a function that returns a Result to determine if the employee
//   may enter the building
fn check_employee_access(employee: &Employee) -> Result<(), String> {
    match employee.status {
        EmployeeStatus::Terminated => return Err("This is not an Employee".to_owned()),
        _  => ()
    }

    match employee.position {
        EmployeePosition::Maintenance => Ok(()),
        EmployeePosition::Marketing => Ok(()),
        EmployeePosition::Mannager => Ok(()),
        _ => Err("Access Denied".to_owned()),
    }
}

// * Print whether the employee may access the building
//   * Must use a function that utilizes the question mark operator to do this
fn print_employee_access(employee: &Employee) -> Result<(), String> {
    check_employee_access(employee)?;
    println!("Allow Access");
    Ok(())
}

fn main() {
    let employees = vec![
        Employee {
            position: EmployeePosition::Mannager,
            status: EmployeeStatus::Actived
        },
        Employee {
            position: EmployeePosition::Marketing,
            status: EmployeeStatus::Actived
        },
        Employee {
            position: EmployeePosition::Maintenance,
            status: EmployeeStatus::Terminated
        },
        Employee {
            position: EmployeePosition::LineSupervisor,
            status: EmployeeStatus::Actived
        },
        Employee {
            position: EmployeePosition::KitchenStaff,
            status: EmployeeStatus::Actived
        },
        Employee {
            position: EmployeePosition::AssemblyTec,
            status: EmployeeStatus::Terminated
        },
    ];

    for empl in employees {
        match print_employee_access(&empl) {
            Err(e) => println!("{:?}", e),
            _ => (),
        }
    }

}