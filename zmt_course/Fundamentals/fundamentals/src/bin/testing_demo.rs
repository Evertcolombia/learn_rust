fn all_caps(word: &str) -> String {
    word.to_uppercase()
}

fn main() {

}

#[cfg(test)]
mod test {

    use crate::*; // use all the functionality in this file scope
    #[test]
    fn check_all_caps() {
        let result = all_caps("hello");
        let expected = String::from("hello");

        assert_eq!(result, expected, "String should be all uppercase");
    }
}