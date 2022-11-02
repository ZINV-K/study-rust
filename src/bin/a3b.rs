// Using while

fn classification(v:[i32; 3]) {
    let l:usize = v.len();
    println!("Array size is {}", l);
    
    let mut i:usize = 0;
    while i < l {
        println!("processing in {:?} times...", i);
        if v[i] < 5 {
            println!("The {:?} is smaller than 5", v[i]);
        } else if v[i] == 5 {
            println!("The {:?} is equal with 5", v[i]);
        } else if v[i] > 5 {
            println!("The {:?} is greater than 5", v[i]);
        }
        i = i + 1;
    }
}

fn main() {
    let values = [3,5,7];

    classification(values);
}