// advance enum 

#[derive(Debug, Clone)]
enum Ticket{
    Backstage(f64, String),
    Vip(f64, String),
    Standard(f64),
}

fn main() {
    let my_tickets: Vec<Ticket> = vec![
        Ticket::Backstage(18.99, "Billy".to_owned()),
        Ticket::Vip(12.99, "Amy".to_owned()),
        Ticket::Standard(6.99),
    ];

    for ticket in &my_tickets {
        match ticket {
            Ticket::Backstage(price, holder) => println!("Backstage ticket Holder: {:?}, price: {:?}", holder, price),
            Ticket::Vip(price, holder) => println!("Vip ticket Holder: {:?}, price: {:?}", holder, price),
            Ticket::Standard(price) => println!("Standard ticket price: {:?}", price),
        }
    }
}
