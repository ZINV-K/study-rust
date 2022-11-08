// Topic: New type pattern
//
// Requirements:
// * Display the selected color of shoes, a shirt, and pants
// * Create and display at least one of each type of clothes and color
//
// Notes:
// * Create a new type for each clothing item that wraps the Color enum
//   * Each new type should implement a `new` function
// * Create a function for each type of clothes (shoes, shirt, pants)
//   that accepts the new type specific to that type of clothing

#[derive(Debug)]
enum Color {
    Black,
    Blue,
    Brown,
    Custom(String),
    Gray,
    Green,
    Purple,
    Red,
    White,
    Yellow,
}

impl Color {
    fn new(color: &str) -> Self {
        match color.to_lowercase().as_ref() {
            "black" => Color::Black,
            "blue" => Color::Blue,
            "brown" => Color::Brown,
            "gray" => Color::Gray,
            "green" => Color::Green,
            "purple" => Color::Purple,
            "red" => Color::Red,
            "white" => Color::White,
            "yellow" => Color::Yellow,
            _ => Color::Custom(color.to_owned()),
        }
    }
}

trait Print {
    fn print(&self) {}
}

#[derive(Debug)]
struct Shoes(Color);
impl Shoes {
    pub fn new(color: Color) -> Self {
        Self(color)
    }
}

impl Print for Shoes {
    fn print(&self) {
        match &self.0 {
            Color::Custom(color) => println!("Shoes Color: {:?}", color),
            _ => println!("Shoes Color: {:?}", self.0),
        }
    }
}

#[derive(Debug)]
struct Shirt(Color);
impl Shirt {
    pub fn new(color: Color) -> Self {
        Self(color)
    }
}

impl Print for Shirt {
    fn print(&self) {
        match &self.0 {
            Color::Custom(color) => println!("Shirt Color: {:?}", color),
            _ => println!("Shirt Color: {:?}", self.0),
        }
    }
}

#[derive(Debug)]
struct Pant(Color);
impl Pant {
    pub fn new(color: Color) -> Self {
        Self(color)
    }
}

impl Print for Pant {
    fn print(&self) {
        match &self.0 {
            Color::Custom(color) => println!("Pant Color: {:?}", color),
            _ => println!("Pant Color: {:?}", self.0),
        }
    }
}

fn print_shoes_color(shoes: Shoes) {
    shoes.print();
}
fn print_shirt_color(shirt: Shirt) {
    shirt.print();
}
fn print_pant_color(pant: Pant) {
    pant.print();
}

fn main() {
    let shoes = Shoes::new(Color::new("red"));
    let shirt = Shirt::new(Color::new("blue"));
    let pant = Pant::new(Color::new("asdf"));

    print_shoes_color(shoes);
    print_shirt_color(shirt);
    print_pant_color(pant);
}
