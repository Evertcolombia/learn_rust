// Iterators are a simpley way to traverse and
// manipulate data structures

fn main() {
    let numbers = vec![1, 2, 3, 4, 5];

    let mut plus_one = vec![];

    for num in numbers {
        plus_one.push(num + 1);
    }

    // the reason touse type anotation in ths case is because
    // iter() and collect() generally operates  any kind of data structures
    let plus_one: Vec<_> = numbers
        .iter() // traverse  the data structure that is concatenated
        .map(|num| num + 1)
        .collect(); // collect the data in a new data structure of the same type

    let new_numbers: Vec<_> = numbers
        .iter() 
        .filter(|num| num <= 3)
        .collect();

    let numbers = vec![1, 2, 3, 4, 5];
    let find_me: Option<i32> = numbers
        .iter()
        .find(|num| num == 4)
    
    let count = numbers
        .iter()
        .count(); // return the number of elements within the the iterator

    let last: Option<i32> = numbers
        .iter()
        .last(); // returns the las telement of the iterator

    let numbers = vec![1, 2, 3, 4, 5];
    let min: Option<i32> = numbers
        .iter()
        .min();
    
    let max: Option<i32> = numbers
        .iter()
        .max();

    let take: Vec<i32> = numbers
        .iter()
        .take(3) // will take the number of elements passes as argument from the data structure concatenate
        .collect();
}