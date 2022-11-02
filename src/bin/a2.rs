// Calculating with functions
// Optional Argument (Parameter)

fn sum(a: i32, b: i32) -> i32{
    a + b
}

fn sub(a: i32, b: i32) -> i32{
    a - b
}

fn remain(a: i32, b: i32) -> i32{
    a % b
}

fn display(a: i32, b: Option<String>){
    if b != None {
        println!("The {} result is {:?}", b.unwrap(), a);
    } else {
        println!("The result is {:?}", a);
    }
}

fn main() {
    let sum:i32 = sum(1,2);
    let sub:i32 = sub(1, 2);

    
    display(sum, Some(String::from("sum")));
    display(sub, Some(String::from("sub")));

    display(remain(6, 2), None);
    display(remain(7, 2), None);
}