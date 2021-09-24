#![allow(unused)]
// This is the root of a library crate with the name ch7-structuring

// in outer module
pub fn get_food() {
    use front_of_house;
    let h1 = front_of_house::Host { name: String::from("Amy") };
    println!("From sub module: {:?}", h1);

    // use a function in a sub sub module.
    front_of_house::serving::take_order();
    let food = back_of_the_house::Appetizer::Soup;
    println!("From back of the house: {:?}", food);

    // use crate::front_of_house::serving; valid
    use self::front_of_house::serving;
    serving::take_order();
    // or bring all public functions under serving into scope.
    use self::front_of_house::serving::take_order;
    take_order();

    // use nested paths to not list the same path multiple times.
    // use front_of_house::{serving, Host};
}

// This exposes
pub use front_of_house::serving::superb_host;

mod front_of_house {

    #[derive(Debug)]
    pub struct Host {
        pub name: String,
    }

    mod hosting {
        fn add_to_waitlist() {}

        fn seat_at_table() {}
    }

    pub mod serving {
        pub mod superb_host {
            pub fn do_thing() {

            }
        }
        pub fn take_order() {}

        fn serve_order() {}

        fn take_payment() {}
    }
}

mod back_of_the_house {

    #[derive(Debug)]
    pub enum Appetizer {
        Soup,
        Salad,
    }
}
