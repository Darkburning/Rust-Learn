#[derive(Debug)]
enum IpAddrKind {
    V4,
    V6,
}
// enum as struct's field
// struct IpAddr {
//     kind: IpAddrKind,
//     address: String,
// }
// enum hold value
// enum IpAddr {
//     V4(String),
//     V6(String),
// }
// enum hold different value
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}
impl Message {
    fn call(&self) {
        println!("Message's function was called!")
    }
}
fn main() {
    let ipv4 = IpAddrKind::V4;
    let ipv6 = IpAddrKind::V6;
    // enum as fn's arguments
    route(ipv4);
    route(ipv6);

    // enum as struct's field
    // let home = IpAddr {
    //     kind: IpAddrKind::V4,
    //     address: String::from("127.0.0.1"),
    // };

    // let loopback = IpAddr {
    //     kind: IpAddrKind::V6,
    //     address: String::from("::1"),
    // };
    // enum hold value
    // let home = IpAddr::V4(String::from("127.0.0.1"));

    // let loopback = IpAddr::V6(String::from("::1"));
    // enum hold different value
    let home = IpAddr::V4(127, 0, 0, 1);

    let loopback = IpAddr::V6(String::from("::1"));

    let message = Message::Quit;
    message.call();

    let quit = Message::Move { x: 123, y: 123 };
    let write = Message::Write(String::from("hello!"));
    let ChangeColor = Message::ChangeColor(1, 2, 3);

    // Option enum
    let some_number = Some(5);
    let some_string = Some("a string");

    let absent_number: Option<i32> = None;

    // compile error: cannot add `Option<i8>` to `i8`
    // let x: i8 = 5;
    // let y: Option<i8> = Some(5);

    // let sum = x + y;
}
// enum as fn's parameters
fn route(ip_kind: IpAddrKind) {
    println!("ip_kind: {:#?} was route!", ip_kind)
}
