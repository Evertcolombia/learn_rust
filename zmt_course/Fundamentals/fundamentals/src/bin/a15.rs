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


// * Use an enum for the tickets with data associated with each variant
enum Ticket {
    Backstage(i32, String),
    Vip(i32, String),
    Standard(i32)
}

impl Ticket {
    fn print(&self) {
        match self {
            Ticket::Backstage(price, owner) => {
                println!("Backtage access: Buyer =  {:?} - price = ${:?}", owner, price);
            },
            Ticket::Vip(price, owner) => {
                println!("VIP access: Buyer =  {:?} - price = ${:?}", owner, price);
            },
            Ticket::Standard(price) => println!("Standard access: price = ${:?}", price),
        }
    }
}


fn main() {
    // * Create one of each ticket and place into a vector
    let buyers = vec![
        Ticket::Backstage(500, "Mercedes Escalante".to_owned()),
        Ticket::Vip(300, "Evert Escalante".to_owned()),
        Ticket::Standard(120),
    ];

    for ticket in buyers {
        ticket.print();
    }
}