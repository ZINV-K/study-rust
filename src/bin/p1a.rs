// project 1 another
use std::{io, collections::HashMap};

#[derive(Debug, Clone)]
struct Bill {
    name: String,
    amount: f64,
}

struct Bills{
    inner: HashMap<String, Bill>
}

impl Bills{
    fn new() -> Self{
        Self { inner: HashMap::new() }
    }

    fn add(&mut self, bill: Bill) {
        // bill이름을 clonning 하는 이유는 우리가 소유한 문자열(<String>)을 가져야하기 때문
        self.inner.insert(bill.name.clone(), bill);
    }

    // &Vec -> Vec 차용하지 않고 않에서 for문으로 새롭게 Vec을 생성했으므로 소유값을 전달함
    fn get_all(&self) -> Vec<Bill> {
        let mut bills = vec![];  // <-- 여기서 소유값을 생성했는데 참조값을 전달하면
                                                // 이 함수의 종료와 함께 참조값이 사라지므로 오류가 발생할 수 있음
        
        // for 다음에 위치한 bill의 위치는 항상 (&)빌려쓰는 자리
        // 빌려 온 값에서 .values()를 호출하더라도 리턴되는 값은 항상 빌려오는 값
        for bill in self.inner.values() {
            // ^ 위에서 빌려왔지만 참조값으로 넣으면 for문과 함께 사라지므로
            // 소유 값(<Bill>)으로 return 하기 위해서 clone함
            // clone을 하기위해서 derive(clone)을 추가해야함
            bills.push(bill.clone());
        }
        // 소유권을 이 함수를 호출한 곳으로 돌려줌
        bills
    }

    fn remove(&mut self, name: &str)-> bool {
        // is_some (T,F) 삭제되었는지 콜백받음
        self.inner.remove(name).is_some()
    }

    fn update (&mut self, name: &str, amount: f64) -> bool {
        // mut 한 값을 돌려줌 Some 또는 None
        match self.inner.get_mut(name) {
            Some(bill) => {
                bill.amount = amount;
                true
            }
            None => false
        }
    }
}

fn get_input() -> Option<String>{
    let mut buffer = String::new();
    while io::stdin().read_line(&mut buffer).is_err() {
        println!("Please enter your data again");
    }
    let input = buffer.trim().to_owned();
    if &input == "" {
        None
    } else {
        Some(input)
    }
}

fn get_bill_amount() -> Option<f64> {
    println!("Amount:");
    loop {
        let input = match get_input() {
            Some(input) => input,
            None => return None,
        };
        if &input == "" {
            return None
        }

        let parsed_input: Result<f64, _> = input.parse();
        match parsed_input {
            Ok(amount) => return Some(amount),
            Err(_) => println!("Please enter a number"),
        }
    }
}

fn add_bill_menu(bills: &mut Bills) {
    println!("Bill name:");
    let name = match get_input() {
        Some(input) => input,
        None => return,
    };
    let amount = match get_bill_amount() {
        Some(amount) => amount,
        None => return,
    };

    let bill = Bill { name, amount };
    bills.add(bill);

    println!("Bill added");
}

// 수정해야되기 때문에 mut을 사용함
fn remove_bill_menu(bills: &mut Bills) {
    for bill in bills.get_all() {
        println!("{:?}", bill);
    }
    println!("Enter bill name to remove:");
    let input = match get_input(){
        Some(input) => input,
        None => return,
    };
    if bills.remove(&input) {
        println!("removed");
    } else {
        println!("bill not found");
    }
}
// 수정해야되기 때문에 mut을 사용함
fn update_bill_menu(bills: &mut Bills) {
    for bill in bills.get_all() {
        println!("{:?}", bill);
    }
    println!("Enter bill name to update:");
    let name = match get_input() {
        Some(name) => name,
        None => return,
    };
    let amount = match get_bill_amount() {
        Some(amount) => amount,
        None => return,
    };
    // String에 &String 참조를 걸어도 &str에서 받을 수 있음
    // &String = &str
    if bills.update(&name, amount) {
        println!("updated");
    } else {
        println!("bill not found");
    }
}

// 조회만 하고 변경하지 않기에 mut을 사용하지 않음
fn view_bills_menu(bills: &Bills) {
    for bill in bills.get_all() {
        println!("{:?}", bill);
    }
}

fn main_menu() {
    fn show() {
        println!("");
        println!("== Manage Bills ==");
        println!("1. Add bill");
        println!("2. View bill");
        println!("3. Remove bill");
        println!("4. Update bill");
        println!("");
        println!("Enter selection:");
    }

    let mut bills = Bills::new();

    loop {
        show();
        let input = match get_input() {
            Some(input) => input,
            None => return,
        };
        match input.as_str() {
            "1" => add_bill_menu(&mut bills), // <--- 수정이 필요해 mut을 사용함
            "2" => view_bills_menu(&bills), // <-- 보기만 할거라 mut을 사용하지 않음
            "3" => remove_bill_menu(&mut bills), // <-- 보기만 할거라 mut을 사용하지 않음
            "4" => update_bill_menu(&mut bills), // <-- 보기만 할거라 mut을 사용하지 않음
            _ => break,
        }
    }
}

fn main() {
    loop {
        main_menu();
    }
}