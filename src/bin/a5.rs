// Using loop

fn main() {
    let mut i:i32 = 0;

    loop {
        println!("Processing {:?} times...", i);
        i = i + 1;
        if i >= 5 {
            break;
        }
    }

    println!("Processed!")
}