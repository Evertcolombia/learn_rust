fn main() {
    let my_name = "polkadot";

    match my_name {
        "Jason" => println!("not my name!"),
        "Evert" => println!("Thats my name!"),
        "Jaidy" => println!("Hello Jaidy"),
        _ => println!("Nice to meet you")
    }
}