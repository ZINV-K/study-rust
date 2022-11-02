// Using enum with struct
enum Drink {
    Chocholate,
    Coffee,
    Tea,
    Juice,
    Ade
}

enum DrinkOption {
    Ice,
    Hot,
}

struct DrinkItem{
    drink: Drink,
    option: DrinkOption,
    price: f64,
    amount: i32,
}

fn order(order: &DrinkItem) {
    match order.drink {
        Drink::Chocholate => show_menu("You choose a chocolate drink!", &order.option),
        Drink::Coffee => show_menu("You choose a coffee drink!", &order.option),
        Drink::Tea => show_menu("You choose a tea drink!", &order.option),
        Drink::Juice => show_menu("You choose a juice drink!", &order.option),
        Drink::Ade => show_menu("You choose a ade drink!", &order.option),
    }
    println!("Price is ${}.", order.price)
}

fn show_menu(message: &str, option: &DrinkOption) {
    let o;
    match option {
        DrinkOption::Hot => o = "hot",
        DrinkOption::Ice => o = "ice", 
    }

    println!("{} And it's {}!", message, o)
}

fn main() {
    let my_order = DrinkItem{
        drink: Drink::Tea,
        option: DrinkOption::Hot,
        price: 2.99,
        amount: 1,
    };

    order(&my_order);

    order(&my_order);
    
    let my_order2 = DrinkItem{
        drink: Drink::Ade,
        option: DrinkOption::Ice,
        price: 3.99,
        amount: 1,
    };
    
    order(&my_order2);
}