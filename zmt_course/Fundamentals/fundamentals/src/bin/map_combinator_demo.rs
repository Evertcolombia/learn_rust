// * map is a method  to perform an operationn faster
// like if were executing a clousure

fn maybe_num() -> Option<i32> {
    Some(45)
}

fn maybe_word() -> Option<String> {
    Some("Evertcolombia".to_owned())
}

fn main() {

    let num = maybe_num().map(|num| num * 2);
    match num {
        Some(n) => println!("{:?}", n),
        _ => println!("None")
    }

    let word = maybe_word()
        .map(|wrd| wrd.to_uppercase());
        //.map(|wrd_len|  wrd_len * 2);
    
    match word {
        Some(w) => println!("{:?}", w),
        _ => println!("None")
    }
}