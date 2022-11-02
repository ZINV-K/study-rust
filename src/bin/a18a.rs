// Result and ?
#[derive(Debug, Clone)]
enum Position{
    Supervisor,
    Manager,
    Crew,
    Staff,
}
#[derive(Debug, Clone)]
enum Status{
    Active,
    Terminate
}

#[derive(Debug, Clone)]
enum Department {
    Maintenance,
    Marketing,
    Assembly,
    Kitchen,
    Line,
}

#[derive(Debug, Clone)]
struct Employee{
    name: String,
    age: i32,
    department: Department,
    position: Position,
    status: Status,
}

impl Employee {
    fn new(name: &str, age: i32, department: Department, position: Position, status:Status) -> Self{
        Self{name: name.to_owned(), age, department, position, status}
    }
}

fn check_authority(employee: &Employee) -> Result<&Employee, String> {
    match employee {
        Employee { department: Department::Maintenance, ..} | Employee { department: Department::Marketing, ..} => return Ok(employee),
        Employee { position: Position::Manager, .. } => return Ok(employee),
        _ => {
            // println!("{:?} is working at not this building.", employee.name);
            Err("Access is denied".to_owned())
        },
    }
}

fn check_status(employee: &Employee) -> Result<&Employee, String> {
    match employee.status {
        Status::Active => return Ok(employee),
        _ => Err("Status is terminated".to_owned()),
    }
}

fn try_access(access: Result<&Employee, String>) -> Result<(), String>{
    let result = access?; // 결과가 ok면 그 안의 값을 반환함
    match check_status(result) {
        Err(e) => println!("Access is denied: {:?}", e),
        // check_status 결과가 ok면 그 안의 값이 반환되어야함
        _ => print_result(check_status(result)?),
    }
    Ok(())
}

fn print_result(employee: &Employee) {
    println!("Welcome {:?}, working at {:?}.", employee.name, employee.department)
}

fn main() {
    let employees = vec![
        Employee::new("Anna", 23, Department::Assembly, Position::Manager, Status::Active),
        Employee::new("Billy", 24, Department::Assembly, Position::Crew, Status::Active),
        Employee::new("Chales", 23, Department::Kitchen, Position::Crew, Status::Terminate),
        Employee::new("Donnie", 25, Department::Kitchen, Position::Staff, Status::Active),
        Employee::new("Felix", 27, Department::Maintenance, Position::Manager, Status::Terminate),
        Employee::new("George", 26, Department::Maintenance, Position::Crew, Status::Active),
        Employee::new("Huston", 28, Department::Marketing, Position::Supervisor, Status::Terminate),
        Employee::new("Jackson", 24, Department::Marketing, Position::Staff, Status::Active),
        Employee::new("Kally", 25, Department::Line, Position::Supervisor, Status::Terminate),
        Employee::new("Lauren", 22, Department::Line, Position::Staff, Status::Active),
    ];

    for employee in &employees {
        try_access(check_authority(employee));
    }

}