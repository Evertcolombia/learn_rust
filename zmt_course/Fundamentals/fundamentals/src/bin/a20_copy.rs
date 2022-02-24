// Topic: User input
//
// Requirements:
// * Verify user input against pre-defined keywords
// * The keywords represent possible power options for a computer:
//   * Off
//   * Sleep
//   * Reboot
//   * Shutdown
//   * Hibernate
// * If the user enters one of the keywords, a message should be printed to
//   the console indicating which action will be taken
//   * Example: If the user types in "shutdown" a message should display such
//     as "shutting down"
// * If the keyword entered does not exist, an appropriate error message
//   should be displayed
//
// Notes:
// * Use an enum to store the possible power states
// * Use a function with a match expression to print out the power messages
//   * The function should accept the enum as an input
// * Use a match expression to convert the user input into the power state enum
// * The program should be case-insensitive (the user should be able to type
//   Reboot, reboot, REBOOT, etc.)


use std::io;

// * Use an enum to store the possible power states
enum PowerOptions {
    Off,
    Sleep,
    Reboot,
    Shutdown,
    Hibernate
}

impl PowerOptions {
    fn create(input: &str) -> Option<PowerOptions> {
        let new_input = input.to_lowercase();

        match new_input.as_str() {
            "off" => Some(PowerOptions::Off),
            "sleep" => Some(PowerOptions::Sleep),
            "reboot" => Some(PowerOptions::Reboot),
            "shutdown" => Some(PowerOptions::Shutdown),
            "hibernate" => Some(PowerOptions::Hibernate),
            _ => None,
        }
    }

    fn print(self) {
        use PowerOptions::*;

        match self {
            Off => println!("Turn Off now"),
            Sleep => println!("Go to Sleep now"),
            Reboot => println!("Reboot now"),
            Shutdown => println!("Shutdown bye bye"),
            Hibernate => println!("Go to hibernate")
        }
    }
}

fn get_input() -> io::Result<String> {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer)?;
    Ok(buffer.trim().to_owned())
}

fn main() {
    let user_input = get_input();
    let mut option: Option<PowerOptions> = None;

    match user_input {
        Ok(word) => option = PowerOptions::create(&word),
        Err(e) => println!("error: {:?}", e),
    }

    match option {
        Some(the_option) => the_option.print(),
        None => println!("None, option does not exists")
    }




    
}