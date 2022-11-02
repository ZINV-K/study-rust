// Using enum type
enum Colors {
    White,
    Red,
    Green,
    Blue,
    Black,
}

fn print_color(c: Colors) {
    match c {
        Colors::White => println!("The color is white"),
        Colors::Red => println!("The color is red"),
        Colors::Green => println!("The color is green"),
        Colors::Blue => println!("The color is blue"),
        Colors::Black => println!("The color is black"),
    }
}
fn main() {
    let color = Colors::Red;
    print_color(color);
}