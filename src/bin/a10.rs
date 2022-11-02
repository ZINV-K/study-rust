// expression variable

enum Num {
    One,
    Two,
    Three,
    Four,
    Five
}

fn console_log(msg: &str) {
    println!("{}", msg);
}

fn main() {
    let my_num: Num = Num::Four;
    let judge: bool = my_num as usize + 1 > 3; // <---
    let print = match judge { // <---
        true => {
            if my_num as usize + 1 == 5 {
                console_log("Too big!");
            } else {
                console_log("big!");
            }
        }
        false => {
            if my_num as usize + 1 == 3 {
                console_log("not bad");
            } else {
                console_log("small");
            }
        }
    };
}