// Vector
struct Numbers {
    value: i32,
}

impl Numbers {
    fn new(value:i32) -> Self {
        Self { value }
    }
}

fn show_len(l:Vec<Numbers>) {
    println!("length is {}.", l.len());
}
fn main() {
    let mut my_numbers: Vec<Numbers> = Vec::new();

    my_numbers.push(Numbers::new(10));
    my_numbers.push(Numbers::new(20));
    my_numbers.push(Numbers::new(30));
    my_numbers.push(Numbers::new(40));
    my_numbers.push(Numbers::new(50));

    for number in &my_numbers {
        match number.value{
            30 => println!("Thirty"),
            _ => println!("{:?}", number.value)
        }
    }

    show_len(my_numbers);
}
