// Functions are used to encapsulate funcionallities

// The follow is an example of a function declaration and execution of the same


/* Declarations syntax:

    
    keyword name (parameters) return type {
        function body
    }
*/

//example:
    fn add(a: i32, b: i32) -> i32 {
        a + b // this is the last line of the function body so, its the return value
    }

//Execution:
    let x = add(1, 1); ---> 2
    ley y = add(3, 0); ---> 3
    let z = add(x, 1); ---> 3