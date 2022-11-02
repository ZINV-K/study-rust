// derive

#[derive(Debug, Clone, Copy)]
enum TicketType{
    Backstage,
    Vip,
    Standard,
}

#[derive(Debug, Clone)]
struct Ticket{
    concert: String,
    kind: TicketType,
    holder: Option<String>,
    price: f64,
}

impl Ticket{
    fn new(concert: &str, kind:TicketType, holder: Option<&str>, price:f64) -> Self{
        match kind {
            TicketType::Backstage | TicketType::Vip => Self { concert:concert.to_owned(), kind, holder: Some(holder.unwrap().to_owned()), price },
            _ => Self { concert:concert.to_owned(), kind, holder: None, price }
        }
        
    }

    // fn show_ticket(self) { <--- Copy 없음으로 에러 발생
    fn show_ticket(&self) {
        println!("{:?}", self);
    }
}

fn main() {
    let my_tickets: Vec<Ticket> = vec![
        Ticket::new("Classic Music", TicketType::Backstage, Some("Micheal"), 18.99),
        Ticket::new("Rock Festival", TicketType::Vip, Some("Marie"), 12.99),
        Ticket::new("EDM Party", TicketType::Standard, Some("Anna"), 6.99),
        Ticket::new("Pop Music", TicketType::Standard, Some("John"), 3.99),
    ];

    let all_tickets = for ticket in &my_tickets {
        ticket.show_ticket();
    };

    let get_backstage = for ticket in &my_tickets {
        match ticket {
            Ticket{ kind:TicketType::Backstage, .. } => println!("Backstage of {:?} ticket for {:?}", ticket.kind, ticket.holder),
            _ => println!("This is not backstage ticket"),   
        } 
    };

    let get_chipper_ticket = for ticket in &my_tickets {
        if ticket.price < 10.0 {
            println!("This ticket of {:?} for {:?} is chipper ticket more than others.", ticket.concert, ticket.holder);
        }
    };
}
