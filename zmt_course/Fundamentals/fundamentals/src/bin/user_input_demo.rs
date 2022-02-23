use std::io;

// io provides an special result where the error, option is yet include
fn get_input() -> io::Result<String> {
    let mut buffer = String::new();
    // get the line from user input, "?" will manage error if this happens
    io::stdin().read_line(&mut buffer)?;
    
    // use trim to avoid the specialcharacters from user input
    // like \n or \r use .trim() method that slices the string
    Ok(buffer.trim().to_owned())
    // Ok(buffer)
}

fn main() {
    let mut all_input = vec![];
    let mut times_input = 0;

    while times_input < 2 {
        match get_input() {
            Ok(words) => {
                all_input.push(words);
                times_input += 1;
            }
            Err(e) => println!("error: {:?}", e),
        }
    }

    for input in all_input {
        println!(
            "input: {:?} - input capitalize: {:?}",
            input,
            input.to_uppercase()
        );
    }
}