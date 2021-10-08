#![allow(unused)]

fn main() {
    test_if_let();
    stack_while_let();
    for_pattern();
    let_is_a_pattern();

    let v = Vector { x: 10, y: 60, w: 6, z: 10 };
    function_parameters_as_patterns(&v);

    match_expression();

    at_bindings();
}

fn at_bindings() {
    enum Message {
        Hello { id: i32 },
    }

    let msg = Message::Hello { id: 5 };

    match msg {
        Message::Hello { id:renamed_as_foo } => println!("HELLOO!! ENUMS!! {}", renamed_as_foo),
    }

    match msg {
        Message::Hello {
            id: id_variable @ 3..=7,
        } => println!("Found an id in range: {}", id_variable),
        Message::Hello { id: 10..=12 } => {
            println!("Found an id in another range")
        }
        Message::Hello { id } => println!("Found some other id: {}", id),
    }

}

struct Vector {
    x: i32,
    y: i32,
    z: i32,
    w: i32,
}

fn match_expression() {
    let x = 1;
    match x {
        1 | 2 => println!("One or Two"),
        _ => println!("No IDEA"),
    }

    match x {
        1..=5 => println!("one through five"),
        _ => println!("something else"),
    }

    let v = Vector { x: 10, y: 60, w: 6, z: 10 };
    match v {
        Vector { x: 14, .. } => println!("X is 14!"),
        Vector { x, y, z, w: _ } => println!("X: {} Y: {} Z: {}", x, y, z),
    }


    // match the first and the last in a tuple
    let (first, .., last) = (2, 4, 8, 5, 10, 24);
    println!("First: {} Last: {}", first, last);


    let num = Some(4);
    // match guards
    match num {
        Some(x) if x < 5 => println!("less than five: {}", x),
        Some(x) => println!("{}", x),
        None => (),
    }
}

fn function_parameters_as_patterns(&Vector { x, y, .. }: &Vector) {
    println!("Vector Destructed in param: {}: {}", x, y);
}


fn let_is_a_pattern() {
    println!("Pattern matching");
    //     let PATTERN = EXPRESSION;

    let v = Vector { x: 10, y: 60, w: 6, z: 10 };
    let Vector { x, y, z, w } = v;

    let (x, y, z) = (1, 2, 3);

    // let (x, y) = (1, 2, "bla");
    // won't compile because the number of items don't match the tuple

    let (x, y, ..) = (1, 2, 3, 4);
    println!("x:{}, y:{}", x, y);
}

fn stack_while_let() {
    let mut stack = vec![2, 3, 5];
    while let Some(num) = stack.pop() {
        println!("Popped: {}", num);
    }
}

fn for_pattern() {
    println!("-----------------------");
    let v = vec![1, 5, 7, 10];
    for (index, value) in v.iter().enumerate() {
        println!("{}:{}", index, value);
    }
}

fn test_if_let() {
    let favorite_color: Option<&str> = None;
    let is_tuesday = false;
    let age: Result<u8, _> = "34".parse();

    if let Some(color) = favorite_color {
        println!("Using your favorite color, {}, as the background", color);
    } else if is_tuesday {
        println!("Tuesday is green day!");
    } else if let Ok(age) = age {
        if age > 30 {
            println!("Using purple as the background color");
        } else {
            println!("Using orange as the background color");
        }
    } else {
        println!("Using blue as the background color");
    }
}
