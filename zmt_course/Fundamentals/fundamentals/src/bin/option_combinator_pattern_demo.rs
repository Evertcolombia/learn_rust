// * option combinators allow you easy  manipulate
// data and mannage options

fn main() {
    let a: Option<i32> = Some(2);
    //let a: Option<i32> = None;
    dbg!(a);
    
    // return true if a is Some result, otherwise false
    let is_some = a.is_some();
    dbg!(is_some);
    
    // return true if a is None result, otherwise false
    let is_none = a.is_none();
    dbg!(is_none);

    // map only works with Some DataType, if is None it will not
    // execute the clousure
    let a_mapped = a.map(|num| num * 5);
    dbg!(a_mapped);

    // filter will keep the data to filter if the clousure is true
    // otherwise it will trhow away the data and return None
    // filter always borrow the data to use in the clousure as parameter
    let a_filtered = a.filter(|num| num == &1);
    dbg!(a_filtered);

    // or_else does not  receive and argument.
    // if the data to test is None, the method will return Some\
    // otherwise it will return the same Data that is tested
    let a_or_else = a.or_else(|| Some(5));
    dbg!(a_or_else);

    // unwrap_or_else does not receive arguments on the clousure to exec
    //  return the default value that is tested
    // otherwise if the value is None return 0
    let unwrapped = a.unwrap_or_else(|| 0);
    dbg!(unwrapped);



}