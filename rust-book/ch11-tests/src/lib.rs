
pub fn add_two(a: i32) -> i32 {
    a + 2
}

pub fn is_smaller_by_two(a: i32, b: i32) -> bool {
    (a + 2) == b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    #[should_panic]
    fn shall_panic() {
        panic!("You shall not pass");
    }

    #[test]
    fn assert_macro() {
        // place something that evals to a boolean in here
        assert!(is_smaller_by_two(2, 4));
    }

    #[test]
    fn eq() {
        assert_eq!(4, add_two(2));
    }

    #[test]
    fn neq() {
        assert_ne!(5, add_two(2));
    }

    #[test]
    #[should_panic]
    fn neq_with_msg() {
        assert_eq!(5, add_two(2), "Thou shall fail");
    }

    #[test]
    fn it_works_with_results_too() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }
    //$ use --show-output : ex:  cargo test -- --show-output
    //to force test run to show standard out. By default, if a test does not fail, stdout is not shown.

    #[test]
    #[ignore]
    fn ignore_this_test() {

    }
}
