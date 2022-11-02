// Tuple

fn tuple(day: &str, temp: i32) -> (&str, i32) {
    (day, temp)
}

fn untuple(wether:(&str, i32)) {
    let (day, temp) = wether;
    
    println!("Wether is {} and temp is {}", day, temp);

    if temp > 5 {
        println!("Oh, it's warm~");
    } else if temp < 5 {
        println!("Oh, it's cold...");
    } else {
        println!("Oh, It's not bad.")
    }
}
fn main() {
    let yesterday = tuple("Cloud", 2);
    let today = tuple("Sunny", 6);
    let tomorrow = tuple("Rainy", 5);

    untuple(yesterday);
    untuple(today);
    untuple(tomorrow);
}