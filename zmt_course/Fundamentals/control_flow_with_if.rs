// Example simple if...else

    let a = 99;

    if a > 99 {
        println!("Big number");
    } else {
        println!("Small number");
    }


// Example with nested if flow control

    let a = 99;

    if a > 99 {
        if a > 200 {
            println!("Huge number");    
        } else {
            println!("Big number");
        }
    } else {
        println!("Small number");
    }

// Example with if ... else if ... else:

    let a = 99;

    // this will works case by case
    if a > 200 {
        println!("Huge number");
    } else if a > 99 {
        println!("Big number");
    } else {
        println!("Small number");
    }

    // this will not work case by case
    if a > 99 {
        println!("Big number");
    } else if a > 200 {
        println!("Huge number");
    } else {
        println!("Small number");
    }
