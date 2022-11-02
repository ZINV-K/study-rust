// impl

enum Kind{
    Dimension,
    Weight,
    Color
}
enum Color{
    Red,
    Blue,
    Green
}

impl Color{
    fn show(&self) {
        match self {
            Color::Red => println!("The color is red."),
            Color::Blue => println!("The color is blue."),
            Color::Green => println!("The color is green."),
            _ => println!("Cannot find a color!")
        }
    }
}

struct Dimension{
    width: f64,
    height: f64,
    depth: f64,
}

impl Dimension{
    fn show(&self) {
        println!("---------- Dimension ----------");
        println!("Width: {}mm", self.width);
        println!("Height: {}mm", self.height);
        println!("Depth: {}mm", self.depth);
        println!("-------------------------------");
    }
}

struct Box{
    dimension: Dimension,
    weight: f64,
    color: Color
}

impl Box{
    fn new(dimension:Dimension, weight: f64, color:Color) -> Self {
        // 인자(변수)명과 Box 구조체의 필드명이 일치하므로 한번만 언급해도 작동
        Self { dimension, weight, color }
    }
    fn show(&self, kind:Option<Kind>) {
        // println!("Request: {:?}", kind);
        match kind {
            Some(Kind::Dimension) => self.dimension.show(),
            Some(Kind::Weight) => println!("Weight is {}kg", self.weight),
            Some(Kind::Color) => self.color.show(),
            _ => {
                self.dimension.show();
                println!("Weight is {}kg", self.weight);
                self.color.show();
            }
        }
    }
}

fn main() {
    let my_box_dimension = Dimension{width: 23.4, height: 56.7, depth: 89.1};
    
    let my_box = Box::new(
        my_box_dimension,
        12.3,
        Color::Red
    );

    my_box.show(Some(Kind::Dimension));
    my_box.show(Some(Kind::Weight));
    my_box.show(Some(Kind::Color));
    my_box.show(None);
}