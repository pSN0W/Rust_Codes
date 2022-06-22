// Topic: Advanced match
//
// Requirements:
// * Print out a list of tickets and their information for an event
// * Tickets can be Backstage, Vip, and Standard
// * Backstage and Vip tickets include the ticket holder's name
// * All tickets include the price
//
// Notes:
// * Use an enum for the tickets with data associated with each variant
// * Create one of each ticket and place into a vector
// * Use a match expression while iterating the vector to print the ticket info

enum TicketType {
    Standard,
    Backstage(String),
    Vip(String),
}

struct Ticket {
    price : f32,
    ticket_type : TicketType,
}

fn main() {
    let tickets = vec![
        Ticket{
            price:50.0,
            ticket_type : TicketType::Standard,
        },
        Ticket{
            price:100.0,
            ticket_type : TicketType::Vip("Pratyaksh".to_owned()),
        },
        Ticket{
            price:100.0,
            ticket_type : TicketType::Vip("Janhavi".to_owned()),
        },
        Ticket{
            price:100.0,
            ticket_type : TicketType::Backstage("Pratyaksh".to_owned()),
        },
        Ticket{
            price:100.0,
            ticket_type : TicketType::Backstage("Janhavi".to_owned()),
        },
    ];
    for ticket in tickets {
        match ticket {
            Ticket{price : p,ticket_type : TicketType::Standard} => println!("Standard ticket of {:?}",p),
            Ticket{price : p,ticket_type : TicketType::Vip(name)} => println!("Vip ticket of {:?} for {:?}",name,p),
            Ticket{price : p,ticket_type : TicketType::Backstage(name)} => println!("Backstage ticket of {:?} for {:?}",name,p),
        };
    }

}
