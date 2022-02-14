enum Access {
    Admin,
    Manager,
    User,
    Guest
}

fn main() {
    // secret file: admins only
    let access_level = Access::Guest;

    // use expression  to determine wheter can acces to file or not
    let can_access_file = match access_level {
        Access::Admin => true,
        _ => false,
    };

    println!("{:?}", can_access_file);
}