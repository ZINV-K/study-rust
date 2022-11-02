// Using IF condition

fn send_message(check:bool) {
    if check {
        println!("Hello");
    } else {
        println!("Goodbye")
    }
}

fn main() {
    let mut check = true;
    send_message(check);
    println!("state: {:?}", check);
    check = false;
    send_message(check);
}