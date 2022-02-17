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
    age: i32,
}

// * Use a function to determine if a customer can make a restricted purchase
impl Customer {
    fn check_age(&self) -> Result<(), String> {
        if self.age < 21 {
            return Err("Purchase denied, muts have 21 years old".to_owned());
        }
        Ok(())
    }
}

fn main() {

    let buyer = Customer {
        age: 21,
    };

    match buyer.check_age() {
        Ok(()) => println!("Purchase Done"),
        Err(e) => println!("{:?}", e)
    }
}