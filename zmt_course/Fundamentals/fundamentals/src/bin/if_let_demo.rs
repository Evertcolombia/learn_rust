enum Color {
    Red,
    Black,
    Yellow
}

fn main() {
    
    let maybe_user: Option<String> = Some("Evert".to_owned());
    let maybe_user: Option<String> = None;

    if let Some(user) = maybe_user {
        println!("user = {:?}", user);
    } else {
        println!("Not user");
    }

    let my_color = Color::Yellow;

    if let Color::Black = my_color {
        println!("it's black");
    } else {
        println!("it's not black");
    }
}