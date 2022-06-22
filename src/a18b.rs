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

enum EmployeeType{
    MaintenanceCrew,
    MarketingDepartment,
    Manager,
    LineSupervisor,
    KitchenStaff,
    AssemblyTechnician,
}

enum Status {
    Terminated,
    Active,
}

struct Employee {
    employee_type : EmployeeType,
    status : Status,
}

// we don't want any datatype for type Ok
fn allowed (employee : &Employee) -> Result<(),String> {
    match employee.status {
        Status::Terminated => return Err("The employee is no longer imployed".to_owned()),
        _ => () //this means do nothing
    }
    match employee.employee_type {
        EmployeeType::MaintenanceCrew => Ok(()),
        EmployeeType::MarketingDepartment => Ok(()),
        EmployeeType::Manager => Ok(()),
        _ => Err("This post is not allowed to access building".to_owned()),
    }
}

fn print_allowed (employee : &Employee) -> Result<(),String> {
    // the question mar operator return the error immideately
    // this can be used for chaining
    let is_allowed = allowed(&employee)?;
    // if no error 
    println!("Access Allowed");
    Ok(())
}

fn main() {
    let new_employee = Employee{
        employee_type : EmployeeType::LineSupervisor,
        status : Status::Active,
    };
    let result = print_allowed(&new_employee);
    match result {
        Err(e) => println!("Error : {:?}",e),
        _ => ()
    };
}
