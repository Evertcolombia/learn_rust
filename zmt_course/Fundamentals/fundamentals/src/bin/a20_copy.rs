use std::io;

enum PowerOptions {
    Off,
    Sleep,
    Reboot,
    Shutdown,
    Hibernate,
}

impl PowerOptions {
    fn new(state: &str) -> Option<PowerOptions> {
        let new_state = state.trim().to_lowercase();

        match new_state.as_str() {
            "off" => Some(PowerOptions::Off),
            "sleep" => Some(PowerOptions::Sleep),
            "reboot" => Some(PowerOptions::Reboot),
            "shutdown" => Some(PowerOptions::Shutdown),
            "hibernate" => Some(PowerOptions::Hibernate),
            _ => None
        }
    }
}

fn print_action(option: PowerOptions) {
    use PowerOptions::*;
    match option {
        Off => println!("Turn OFF"),
        Sleep => println!("Just Sleep"),
        Reboot => println!("Will Reboot"),
        Shutdown => println!("Shutting Down"),
        Hibernate => println!("Hibernate Righ Now"),
    }
}

fn main() {
    let mut buffer = String::new();
    let user_input_status = io::stdin().read_line(&mut buffer);

    if user_input_status.is_ok() {
        match PowerOptions::new(&buffer) {
            Some(state) => print_action(state),
            None => println!("invalid power state")
        }
    } else {
        println!("Error reading input");
    }


}

