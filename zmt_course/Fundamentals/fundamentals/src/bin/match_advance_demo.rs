enum Discount {
    Percent(i32),
    Flat(i32),
}

struct Ticket {
    event: String,
    price: i32,
}

fn main() {
    let n = 3;

    match n  {
        3 => println!("Three!"),
        other => println!("number: {:?}", other), // other means like default case
    }

    let flat = Discount::Flat(2);

    match flat {
        Discount::Flat(2) => println!("flat 2"),
        Discount::Flat(amount) => println!("flat discount of: {:?}", amount), // any other values on Discount::Flat()
        _ => (), // --> ignore the rest of the values in the enum
    }

    let concert = Ticket {
        event: String::from("Concert"),
        price: 50,
    };

    match concert {
        Ticket {price: 50, event} => println!("event @ 50 = {:?}", event),
        Ticket {price, ..} => println!("price: {:?}", price), 
        // .. means ignore the rest of the fields in the struct
    }
}