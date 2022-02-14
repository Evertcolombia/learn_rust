struct Book {
    pages: i32,
    rating: i32,
}

/*
Example of moved ownership
 
- A function o code block is the ownership of his variables
    until they are moved to another owner, or until the scope of this function ends

- When in a function the ownership of a value is moved to another function, It can not
    use again this value, because yet its not the ownership

fn display_page_count(book: Book) { // -> is receiving a property to be the new ownership
    println!("pages = {:?}", book.pages);
}

fn display_rating(book: Book) { // -> is receiving a property to be the new ownership
    println!("rating = {:?}", book.rating);
}

fn main() {
    let book = Book {
        pages: 350,
        rating: 8,
    };

    // -> here the ownershing is been moved to the display_page_count function
    // scope
    display_page_count(book); 

    // -> now we'll get an error because the book variable no longer belongs to this scope
    // it means, it does not exist in this scope to use again with other function
    display_rating(book);
}
*/


/*
Example ow Borrow a value

adding an '&' when its necessary means it will borrow a reference to the value that
the & precedes
*/

// -> with the & is receiving a reference to a book value, it means its receiving a borrow value
fn display_page_count(book: &Book) {
    println!("pages = {:?}", book.pages);
}

// -> with the & is receiving a reference to a book value, it means its receiving a borrow value
fn display_rating(book: &Book) {
    println!("rating = {:?}", book.rating);
}

fn main() {
    let book = Book {
        pages: 350,
        rating: 8,
    };

    // -> here the value in book is been borrow to this function
    // and main scope stil will be the owner
    display_page_count(&book); 

    // now we'll not get an error because main continue being the ownership
    // to the value in book
    display_rating(&book);
}


// using borrow instead moved the owner of a value, will perfomance the program because
// it will not have to copy and create a new value under the hood

// instead of that it will borrow a reference of the memory adress, wich have a better performance
// for the compiler
