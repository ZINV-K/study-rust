// Using match 2

fn classification(v:i32) {
    let r = v % 2;
    match r {
        0 => message(v, Some(true)),       
        _ => message(v, Some(false)),       
    }
}

fn message(v:i32, c:Option<bool>) {
    match c {
        Some(true) => println!("{:?} is even.", v),
        _ => println!("{:?} is odd.", v),
    }
}

fn main() {
    let values:[i32; 10] = [0,1,2,3,4,5,6,7,8,9];

    let mut i:usize = 0;
    while i < values.len() {
        classification(values[i]);
        i = i + 1;
    }
}