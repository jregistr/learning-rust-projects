#![allow(unused)]


use crate::polymorph::standard_components::{Button, TextField};
use crate::polymorph::base_gui::{Screen, Draw};

struct DropDownMenu {
    options: Vec<String>,
    selected: Option<String>,
}

impl base_gui::Draw for DropDownMenu {
    fn draw(&self) {
        // draw that dropdown menu
    }
}

fn make_awesome_ui() {
    let ui_elems: Vec<Box<dyn Draw>> = vec![
        Box::new(Button { height: 10, label: String::from("This is button"), width: 30 }),
        Box::new(TextField { default_text: None }),
        Box::new(Button { height: 5, label: String::from("This is also button"), width: 15 }),
        Box::new(DropDownMenu { options: vec![String::from("Do the Thing"), String::from("Julie")], selected: None }),
    ];

    let screen = Screen { component: ui_elems };
}

pub mod base_gui {
    pub trait Draw {
        fn draw(&self);
    }

    pub struct Screen {
        // trait object
        pub component: Vec<Box<dyn Draw>>,
    }

    impl Screen {
        pub fn run_ui(&self) {
            for component in self.component.iter() {
                component.draw();
            }
        }
    }
}

pub mod standard_components {
    use crate::polymorph::base_gui::Draw;

    pub struct Button {
        pub width: i32,
        pub height: i32,
        pub label: String,
    }

    impl Draw for Button {
        fn draw(&self) {
            // Some impl
        }
    }

    pub struct TextField {
        pub default_text: Option<String>,
    }

    impl Draw for TextField {
        fn draw(&self) {
            // draw that text field
        }
    }
}
