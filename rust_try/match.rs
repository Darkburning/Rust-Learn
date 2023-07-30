#[derive(Debug)] // 这样可以立刻看到州的名称
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}
// bind value pattern match
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}

// fn plus_one(x: Option<i32>) -> Option<i32> {
//     // compile error:non-exhaustive patterns: `None` not covered
//     match x {
//         // None => None,
//         Some(i) => Some(i + 1),
//     }
// }

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}
// _通配符
fn match_point(point: u8) {
    match point {
        1 => {
            println!("point 1")
        }
        2 => {
            println!("point 2")
        }
        3 => {
            println!("point 3")
        }
        _ => {
            println!("other point not between 1 and 3")
        }
    }
}

fn main() {
    println!("value_in_cents: {}", value_in_cents(Coin::Penny));
    let Alaska_coin = Coin::Quarter(UsState::Alaska);
    println!("value_in_cents: {}", value_in_cents(Alaska_coin));

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    let point_3 = 3u8;
    let point_255 = 255u8;
    match_point(point_3);
    match_point(point_255);

    // if let syntax sugar
    // let some_u8_value = Some(0u8);
    let some_u8_value = Some(3u8);
    match some_u8_value {
        Some(3) => println!("normal match: three"),
        _ => {
            println!("normal match: other")
        }
    }

    if let Some(3) = some_u8_value {
        println!("if let: three");
    } else {
        println!("if let: other");
    }
}
