enum Message {
    Quit,
    Move { x: i32, y: i32},
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {

    }
}

fn main() {
    let _none = Message::Quit;
    let _an_struct = Message::Move { x: 10, y: 20 };
    let _tp_struct = Message::ChangeColor(255, 255, 255);

    let m = Message::Write(String::from("hello"));
    m.call();
}
