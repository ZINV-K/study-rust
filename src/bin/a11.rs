// ownership
enum Grocery {
    Lamp,
    Chair,
    Soap,
    Towel,
    Perfume,
}

struct Item {
    id: i32,
    item: Grocery,
    quantity: i32,
}

fn display_item(item: &Item) {
    println!("{}", item.id);
    let msg = "is placed in your order.";
    match item.id {
        0 => print_msg(msg, item, Some("Lamp"),  Some("order")),
        1 => print_msg(msg, item, Some("Chair"), Some("order")),
        2 => print_msg(msg, item, Some("Soap"),  Some("order")),
        3 => print_msg(msg, item, Some("Towel"), Some("order")),
        4 => print_msg(msg, item, Some("Perfume"), Some("order")),
        _ => print_msg("is this item?", item, Some("What"),  None),
    };
}

fn display_quantity(item: &Item) -> i32{
    print_msg("quantity ordered.", item, None, Some("quantity"));
    return item.quantity;
}

fn print_msg(msg:&str, item:&Item, name: Option<&str>, kind:Option<&str>) {
    let k = kind.as_deref().unwrap();
    println!("{}", k);
    match k {
        "order" => println!("{:?} {}", name.unwrap(), msg),
        "quantity" =>  println!("{} {}", item.quantity, msg),
        _ => println!("Nothing ordered.")
    }
}

fn main() {
    let order = Item{
        id: Grocery::Chair as i32,
        item: Grocery::Chair,
        quantity: 2,
    };

    display_item(&order);
    display_quantity(&order);
}