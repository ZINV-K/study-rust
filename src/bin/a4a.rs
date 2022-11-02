// Using match

fn classification(v:&str) {
    match v {
        "pear" => message(v, Some(true)),       
        _ => message(v, Some(false)),       
    }
}

fn message(v:&str, c:Option<bool>) {
    match c {
        Some(true) => println!("{:?} is my favourite fruit!", v),
        _ => println!("{:?} is not my favorite fruit.", v),
    }
}

fn main() {
    let values:[&str; 4] = ["apple","banana","grape","pear"];

    let mut i:usize = 0;
    while i < values.len() {
        classification(values[i]);
        i = i + 1;
    }
}