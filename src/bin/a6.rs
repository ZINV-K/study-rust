// Using while

fn main() {
    let mut count: i32 = 5;

    while count > 0 {
        println!("Launch after {} seconds...", count);
        count = count - 1;
    }
    println!("Let's launch it");
}