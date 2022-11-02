// Making functions

fn show_first_name (a: &str) -> &str{
    println!("first name is {:?}", a);
    return a;
}

fn show_last_name (b: &str) -> &str{
    println!("last name is {:?}", b);
    return b;
}

fn main () {
    let mut first_name:&str = "Joshua";
    let mut last_name:&str = "Kim";

    println!("My name is {:?} {:?}", show_first_name(first_name), show_last_name(last_name));
    println!("My name is {} {}", show_first_name(first_name), show_last_name(last_name));
}
