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
    Backstage(i64),
    Vip(i64),
    Standard(i64),
}

struct Buyer {
    ticket: Ticket,
    buyer_name: String,
}

fn main() {

    // * Create one of each ticket and place into a vector
    let buyers = vec![
        Buyer {
            ticket: Ticket::Backstage(500),
            buyer_name: "Santiago Arboleda".to_owned(),
        },
        Buyer {
            ticket: Ticket::Vip(300),
            buyer_name: "Cristian Gomez".to_owned(),
        },
        Buyer {
            ticket: Ticket::Standard(100),
            buyer_name: String::from("Oscar Tapias"),
        }
    ];

    for ticket in buyers {
        // * Use a match expression while iterating the vector to print the ticket info
        match ticket {
            Buyer {ticket: Ticket::Backstage(amount), buyer_name} => println!("Owner: {:?} - Price: {:?}", buyer_name, amount),
            Buyer {ticket: Ticket::Vip(amount), buyer_name} => println!("Owner: {:?} - Price: {:?}", buyer_name, amount),
            Buyer {ticket: Ticket::Standard(amount), ..} => println!("Price: {:?}", amount)
        }
    }
}