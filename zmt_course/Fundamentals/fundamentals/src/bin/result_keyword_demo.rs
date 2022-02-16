#[derive(Debug)]
enum MenuChoice {
    StartMenu,
    Start,
    Quit,
}

fn get_choice(input: &str) -> Result<MenuChoice, String> {
    match input {
        "mainMenu" => Ok(MenuChoice::StartMenu),
        "startMenu" => Ok(MenuChoice::Start),
        "quit" => Ok(MenuChoice::Quit),
        _ => Err("Menu choice not found".to_owned()),
    }
}

fn print_choice(choice: &MenuChoice) {
    println!("Choice = {:?}", choice);
}

fn pick_choice(input: &str) -> Result<(), String> {
    // the ? "question mark" will evaluate if the result is Ok or Er
    // and return the correct option unwrapped of the result
    let choice: MenuChoice = get_choice(input)?;
    print_choice(&choice);
    Ok(()) // empty () means it will return nothing type
}

fn main() {
    //let choice = get_choice("mainMenu");
    //print_choice(&choice);

    // looks cleanner if use the type notation
    //let choice: Result<MenuChoice, _> = get_choice("quit");

    // print the result with ok or err wrapper
    //println!("choice: {:?}", choice);

    // get and print the reulst out of the wrapper
    /*match choice {
        Ok(the_choice) => print_choice(&the_choice),
        Err(e) => println!("error = {:?}", e),
    }*/
    let choice = pick_choice("mainMenu");
    println!("Choice value = {:?}", choice);
}