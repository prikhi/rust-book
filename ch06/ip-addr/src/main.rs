fn main() {
    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));
    route(home);
    route(loopback);
    let m = Message::Write(String::from("hello world"));
    m.call();
    option_section();
}

enum IpAddr {
    // V4,
    // V6,
    //
    //V4(String),
    //V6(String),
    V4(u8, u8, u8, u8),
    V6(String),
}

fn route(_ip_type: IpAddr) {}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {}
}

fn option_section() {
    let _some_number = Some(5);
    let _some_string = Some("a string");
    let _absent_number: Option<i32> = None;
}
