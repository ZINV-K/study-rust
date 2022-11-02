// project 1
use std::{fmt, vec};
use std::{collections::HashMap, io};

#[derive(Debug, Clone, Copy)]
enum Item{
    BTC,
    ETH,
    SOL,
    MECA
}

impl Item {
    fn show(&self) -> String {
        use Item::*;
        match &self{
            BTC => return "BTC".to_owned(),
            ETH => return "ETH".to_owned(),
            SOL => return "SOL".to_owned(),
            MECA => return "MECA".to_owned(),
        }
    }
}

impl fmt::Display for Item {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
        // or, alternatively:
        // fmt::Debug::fmt(self, f)
    }
}

#[derive(Debug, Clone)]
struct Bill{
    symbol: Item,
    amount: String,
}

impl Bill {
    fn new (symbol:Item, amount:String) -> Self {
        Self {symbol, amount}
    }
}

fn input() -> Result<String, String> {
    let mut value = String::new();
    io::stdin().read_line(&mut value);
    Ok(value.trim().to_owned())
}

fn buy_item(customers: &mut HashMap<String, Vec<Bill>>) {
    println!("----------------------------------------------");
    println!("What item do you want to buy?");   

    let item = input();
    let word = item.as_ref().unwrap().to_lowercase();

    if item.is_ok() {

        let get_symbol = match_item(&word);

        match get_symbol {

            Ok(symbol) => {
                let s = symbol.show();
                let (v, a) = check_amount(&s).unwrap();
                match v.as_str() {
                    "back" => return,
                    _ => {
                        let amount =  a.unwrap();
                        println!("{:?}: {:?}", symbol, amount);

                        let customer = get_customer();
                        if customer.is_ok() {

                            let name = customer.unwrap();
                            let new_bill = Bill::new(symbol, amount.to_owned());

                            match customers.get(&name) {
                                Some(v) => {
                                    customers.entry(name).and_modify(|e| { e.push(new_bill) });
                                },
                                None => {
                                    customers.insert(name, vec![new_bill]);
                                },
                            }
                        } else {
                            return
                        }
                    }
                }
            },
            Err(e) => {
                println!("{}", e);
                return
            },
        };
    } else if word == "back" {
        return
    }

        // if customers.get(&word).is_some() {
        //     println!("Hi, {:?}", word);
        //     customers.get(&word);
        // } else {
        //     println!("There is no user that used that name.");
        // }
}

fn match_item(item: &str) -> Result<Item, String> {
    loop {
        let item = item.to_lowercase();
        match item.as_str() {
            "btc" => return Ok(Item::BTC),
            "eth" => return Ok(Item::ETH),
            "sol" => return Ok(Item::SOL),
            "meca" => return Ok(Item::MECA),
            _ => {
                return Err("There is no item like that.".to_owned())
            },
        }
    }
}

fn check_amount(item: &String) -> Result<(String, Option<String>), String>{
    println!("Choose: {:?}", &item);
    println!("----------------------------------------------");
    println!("Please type your amount...");

    let amount = input();

    if amount.is_ok() {
        let amount = amount.unwrap();
        match amount.as_str() {
            "back" => return Ok(("back".to_owned(), None)),
            _ => return Ok(("btc".to_owned(), Some(amount))),
        }
    } else {
        Err("This isn't looks like a number.".to_owned())
    }
}

fn get_customer() -> Result<String, ()> {
    println!("----------------------------------------------");
    println!("Please enter your name...");

    let name = input();
    if name.is_ok() {
        return Ok(name.unwrap())
    } else {
        println!("Something wrong.");
        return Err(())
    }
}

fn show_market(customers: &mut HashMap<String, Vec<Bill>>) {
    println!("----------------------------------------------");
    println!("Welcome to Crypto Market");
    println!("----------------------------------------------");
    println!("Market");
    println!("- BTC");
    println!("- ETH");
    println!("- SOL");
    println!("- MECA"); 

    loop {
        buy_item(customers);
        println!("----------------------------------------------");
        println!("Do you want continue to buy? (y/n)");
        
        let answer = input();
        let user_answer = answer.as_ref().unwrap().to_lowercase();
        
        if answer.is_ok() {
            match user_answer.as_str() {
                "y" => (),
                "n" => {
                    return;
                },
                "back" => {
                    return;
                },
                _ => ()
            }
        } else {
            return;
        }
    }
}

fn show_bills(customers: &mut HashMap<String, Vec<Bill>>) {
    println!("----------------------------------------------");
    println!("Please enter your name...");

    let name = input();
    let key = name.as_ref().unwrap();
    if name.is_ok() {
        let bills = customers.get(key);
        match &bills {
            Some(value) => {
                println!("----------------------------------------------");
                println!("{:?}'s bill list", key);

                println!("----------------------------------------------");
                for bill in value.iter(){
                    println!("{:?}", bill);
                }
                println!("----------------------------------------------");
                get_bill_with_index(key, customers);
            },
            None => {
                println!("There is no bill list of {:?}", key);
            },
        }
    }

}

fn get_bill_with_index(customer:&str, customers: &mut HashMap<String, Vec<Bill>>) {
    loop {
        println!("Do you want to edit or delete your bill? (y/n)");
        
        let user_input = input();
        let text = user_input.as_ref().unwrap().to_lowercase();
        
        if user_input.is_ok() {
            match text.as_str() {
                "y" => {
                    println!("----------------------------------------------");
                    println!("Select bill index number of you want choose.");
                    
                    let user_input = input();
                    let index = user_input.as_ref().unwrap().to_string().parse::<i32>().unwrap() - 1;
                    let bills = customers.get(customer);
                    match bills {
                        Some(value) => {
                            if value.len() < index as usize {
                                println!("There is no {:?}th bill", index);
                            } else if value.len() > 0 && index >= 0{
                                println!("----------------------------------------------");
                                println!("Selected Bill:");
                                println!("{:?}", value[index as usize]);
                                println!("- Edit");
                                println!("- Delete");
                                println!("- Back");

                                let user_input = input();
                                let text = user_input.as_ref().unwrap().to_lowercase();
                                if user_input.is_ok() {
                                    match text.as_str() {
                                        "edit" => {
                                            
                                            println!("----------------------------------------------");
                                            println!("Please select property you want to edit.");
                                            println!("{:?}", value[index as usize]);
                                            println!("----------------------------------------------");
                                            println!("- Symbol");
                                            println!("- Amount");

                                            let user_input = input();
                                            let text = user_input.as_ref().unwrap().to_lowercase();
                                            if user_input.is_ok(){
                                                match text.as_str() {
                                                    "symbol" => {
                                                        customers.entry(customer.to_owned()).and_modify(|e| {
                                                            println!("----------------------------------------------");
                                                            println!("Select symbol you want.");
                                                            println!("----------------------------------------------");
                                                            println!("Market");
                                                            println!("- BTC");
                                                            println!("- ETH");
                                                            println!("- SOL");
                                                            println!("- MECA");
                                                            
                                                            let user_input = input();
                                                            let text = user_input.as_ref().unwrap().to_lowercase();
                                                            
                                                            let mut symbol:Item;
                                                            if user_input.is_ok(){
                                                                match text.as_str() {
                                                                    "btc" => symbol = Item::BTC,
                                                                    "eth" => symbol = Item::ETH,
                                                                    "sol" => symbol = Item::SOL,
                                                                    "meca" => symbol = Item::MECA,
                                                                    _ => {
                                                                        println!("There is no item like that.");
                                                                        return
                                                                    },
                                                                };
                                                                e[index as usize] = Bill{symbol: symbol, amount: e[index as usize].amount.to_string()};
                                                                println!("----------------------------------------------");
                                                                println!("Bill's symbol edited.");
                                                                println!("{:?}", &e[index as usize]);
                                                                return
                                                            } else {
                                                                return
                                                            };
                                                        });
                                                    }
                                                    "amount" => {
                                                        customers.entry(customer.to_owned()).and_modify(|e| {
                                                            println!("----------------------------------------------");
                                                            println!("Enter amount you want.");
                                                            
                                                            let user_input = input();
                                                            let amount = user_input.as_ref().unwrap();
                                                            
                                                            if user_input.is_ok(){
                                                                e[index as usize] = Bill{symbol: e[index as usize].symbol, amount: amount.to_string()};
                                                                println!("----------------------------------------------");
                                                                println!("Bill's amount edited.");
                                                                println!("{:?}", e[index as usize]);
                                                                return
                                                            } else {
                                                                return
                                                            };
                                                        });
                                                        println!()
                                                    },
                                                    _ => {
                                                        println!("There is no property like that.");
                                                    }
                                                }
                                            }
                                        },
                                        "delete" => {
                                            customers.entry(customer.to_owned()).and_modify(|e| {
                                                e.remove(index as usize);
                                            });
                                        },
                                        _ => {},
                                    }
                                }

                            } else {
                                println!("There is no bills.");
                            }
                        },
                        None => println!("There is no bills."),
                    };
                },
                "n" => return,
                _ => {
                    println!("----------------------------------------------");
                    println!("Please type your answer. (y/n)")
                },
            }
        }
        println!("----------------------------------------------");
    }
}

fn main() {
    let mut customers:HashMap<String, Vec<Bill>> = HashMap::new();

    loop {
        println!("----------------------------------------------");
        println!("Welcome to Bill Management System");
        println!("----------------------------------------------");
        println!("Please select option...");
        println!("- View: Show list of bills.");
        println!("- Buy: Show list of markets.");
        println!("- Quit: Exit from this program.");
        println!("----------------------------------------------");
        

        let choice = input();
        let value = choice.as_ref().unwrap().to_lowercase();
        if choice.is_ok() {
            match value.as_str() {
                "quit" => break,
                "view" => show_bills(&mut customers),
                "buy" => show_market(&mut customers),
                _ => println!("There is no menu like {:?}.", value),
            }
        }


        
    }
}