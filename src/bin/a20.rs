// input io
use std::{io, collections::{HashMap}};

#[derive(Debug, Clone, Copy)]
enum PowerState{
    Off,
    Sleep,
    Reboot,
    Shutdown,
    Hibernate
}

impl PowerState {

    // 옵션에 없는 항목을 입력할 경우 Option을 통해 None을 반환해 생략
    fn new(state: &str) -> Option<PowerState>{
        let state = state.trim().to_lowercase();
        //as_str()로 String을 &str으로 변경
        return match state.as_str(){
            "off" => return Some(PowerState::Off),
            "sleep" => return Some(PowerState::Sleep),
            "reboot" => return Some(PowerState::Reboot),
            "shutdown" => return Some(PowerState::Shutdown),
            "hibernate" => return Some(PowerState::Hibernate),
            _ => None,
        };
    }

    fn action(&self) {
        use PowerState::*;
        match self {
            Off => println!("Turning Off..."),
            Sleep => println!("Getting in sleep..."),
            Reboot => println!("Rebooting now..."),
            Shutdown => println!("Shutting down..."),
            Hibernate => println!("Hibernating..."),
        }
    }
}

fn get_input() -> io::Result<String> {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer)?;
    Ok(buffer.trim().to_owned())
}

fn main() {

    let mut my_words:Vec<String> = Vec::new();
    let mut input_times = 0;

    while input_times < 2 {
        println!("Please enter your option...");

        let mut buffer = String::new();
        let mut input_state = io::stdin().read_line(&mut buffer);

        if input_state.is_ok() {
            match PowerState::new(&buffer) {
                Some(state) => state.action(),
                None => println!("There is no option like that"),
            }
            my_words.push(buffer);
            input_times += 1;
        } else {
            println!("Please type again...");
        }
    }

    for word in my_words {
        println!("{:?}", word);
    }

    // println!("Please enter your option...");
    // while input_times < 2 {
    //     match get_input() {
    //         Ok(word) => {
    //             let my_choice = PowerState::new(&word).unwrap();
    //             my_choice.action();
    //             input_times += 1;
    //         },
    //         Err(e) => println!("Error: {:?}", e),
    //     }
    // }
}