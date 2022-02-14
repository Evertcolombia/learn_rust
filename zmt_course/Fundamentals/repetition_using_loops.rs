// rust have multiple types of loops

// "loop" -> the weykor loop will create an infinite loop

    let mut a = 0;

    loop {
        if a == 5 { // if it is true so the infinite loop will be break
            break;
        }

        println!("{:?}", a);
        a = a + 1;
    }


// "while" -> conditional loop

    let mut a = 0;

    while a != 5 {
        println!("{:?}", a);
        a = a + 1;
    }


