#![allow(unused)]

fn main() {
    // both enum variants are namespaced under ts kind. We can also import them into scope.
    use IpAddrKind::*;

    let four_kind = V4;
    let v6 = V6;
    let v6_2 = IpAddrKind::V6;

    println!("{:?}", four_kind);
    println!("{:?}", v6);
    println!("{:?}", v6_2);

    // one way to add data alongside our enum is using a struct. This is not
    // necessary however since rust enums are like algebraic data types. But
    // let's see an example anyhow.
    #[derive(Debug)]
    struct IpAddressStruct {
        kind: IpAddrKind,
        address: String,
    }

    let local_host = IpAddressStruct {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };
    println!("{:?}", local_host);

    // a better way is to associate the appropriate data within the enum variant
    // here, ipv4 has 4 byte values and ipv6's internal data is represented as a string
    // for both of our variants define their data similarly to tuple structs.
    #[derive(Debug)]
    enum IpAddress {
        V4(u8, u8, u8, u8),
        V6(String),
    }

    let v4 = IpAddress::V4(0, 0, 0, 0);
    let v6 = IpAddress::V6(String::from("::1"));
    println!("V4: {:?} --- V6: {:?}", v4, v6);

    // our enum variants can also have named fields like structs.
    #[derive(Debug)]
    enum Color {
        RGB { r: u8, g: u8, b: u8 },
        HEX(String),
    }

    let rgb = Color::RGB { r: 240, g: 120, b: 100 };
    let hex = Color::HEX(String::from("#000"));
    println!("RGB: {:?} -- HEX: {:?}", rgb, hex);

    // another nice similarity between enums and structs if we can define methods for enums as well

    impl Color {
        fn to_str(&self) -> String {
            match self {
                Color::RGB { r, g, b } => format!("{}:{}:{}", r, g, b),
                Color::HEX(hex) => hex.to_string()
            }
        }
    }

    println!("Color converted to str: {}", rgb.to_str());

    /*Match*/

    enum Coin {
        Penny,
        Nickle,
        Dime,
        Quarter(String),
        Dollar,
    }

    impl Coin {
        fn value_in_cents(&self) -> u8 {
            match self {
                Coin::Penny => 1,
                Coin::Nickle => 5,
                Coin::Dime => 10,
                Coin::Quarter(a) if a == "New York" => {
                    println!("New Yoooooooork! Yeeah {}", a);
                    25
                }
                Coin::Quarter(state) => 25,
                Coin::Dollar => 100
            }
        }
    }
    let c = Coin::Dollar;
    println!("in cents {}", c.value_in_cents());
    let ny_q = Coin::Quarter(String::from("New York"));
    println!("in cents {}", ny_q.value_in_cents());

    if let Coin::Quarter(s) = ny_q {
        println!("If let, and yes it's a NY quarter.");
    }
}

#[derive(Debug)]
enum IpAddrKind {
    V4,
    V6,
}

