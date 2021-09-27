use ch11_tests::add_two;

#[test]
fn it_adds_two_integration() {
    assert_eq!(4, add_two(2));
}

// each file under tests is its own test crate