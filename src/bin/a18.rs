// Result and ?

#[derive(Debug, Clone)]
struct Customer{
    name: Option<String>,
    age: i32,
}

impl Customer{
    fn new(name: Option<&str>, age:i32) -> Self{
        match name {
            Some(name) => Self{name: Some(name.to_owned()), age},
            None => Self{name: None, age }, 
        }
    }
}

fn check_authority(customer:&Customer) -> Result<Option<String>, String> {
    if customer.age >= 18 {
        Ok(customer.name.to_owned())
    } else {
        Err("This customer cannot buy this stuff.".to_owned())
    }
}

fn try_purchase(customer: &Customer) -> Result<(), String> {
    let customer_name = check_authority(customer)?;
    print_result(customer_name);
    Ok(())
}

fn print_result (result:Option<String>) {
    println!("{:?} is can buy this stuff", result.unwrap());
}

fn main() {
    let customers = vec![
        Customer::new(None, 6),
        Customer::new(None, 12),
        Customer::new(Some("Jason"), 18),
        Customer::new(Some("Amy"), 24),
        Customer::new(Some("Morgan"), 32),
    ];

    for customer in &customers {
        try_purchase(customer);
    }
}