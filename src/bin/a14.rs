// String and &str
struct Person{
    name: String,
    age: i32,
    color: String,
}

impl Person {
    fn new(name: &str, age:i32, color:String) -> Self{
        Self{name: name.to_owned(), age, color}
    }

    fn show_info(&self) {
        println!("{:?} is {:?} years old, and likes {:?}", self.name, self.age, self.color);
    }
}

fn view_person(name: &str) {
    println!("{:?} is age is over 10.", name);
}
fn main() {
    let people = vec![
        Person::new("Andre", 8, "Red".to_owned()),
        Person::new("Julia", 9, "Blue".to_owned()),
        Person::new("Sara", 10, "Green".to_owned()),
        Person::new("Chales", 11, "Black".to_owned()),
        Person::new("Anna", 12, "White".to_owned()),
    ];

    for p in people {
        if p.age <= 10 {
            p.show_info()
        } else {
            view_person(&p.name);
        }
    }
}
