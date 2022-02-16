// Topic: Result
//
// Requirements:
// * Determine if a customer is able to make a restricted purchase
// * Restricted purchases require that the age of the customer
//   is at least 21
//
// Notes:
// * Use a struct to store at least the age of a customer
// * Use a function to determine if a customer can make a restricted purchase
// * Return a result from the function
// * The Err variant should detail the reason why they cannot make a purchase



// * Use a struct to store at least the age of a customer
struct Customer {
    name: Option<String>,
    age: i32,
}

// * Use a function to determine if a customer can make a restricted purchase
fn make_purchase(customer: &Customer) -> Result<(), String> {
    if customer.age >= 21 {
        Ok(())
    } else {
        Err("Resticted Purchase : Must be older than 21 years".to_owned())
    }
}
// * Return a result from the function
// * The Err variant should detail the reason why they cannot make a purchase

fn main() {
    let customers = vec![
        Customer {
            name: Some("Evert Escalante".to_owned()),
            age: 27,
        },
        Customer {
            name: None,
            age: 18,
        },
    ];
    
    for customer in customers {
        let res = make_purchase(&customer);

        match customer.name {
            Some(name) => println!("Customer name: {:?}", name),
            None => println!("Anonymous customer"),
        }

        match res {
            Ok(()) => println!("Purchase Done"),
            Err(e) => println!("error = {:?}", e),
        }
    }
}    