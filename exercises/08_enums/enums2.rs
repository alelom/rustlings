// enums2.rs
//
// Execute `rustlings hint enums2` or use the `hint` watch subcommand for a
// hint.

// I AM NOT DONE
struct Echo(String);
struct ChangeColor(i32, i32, i32);

#[derive(Debug)]
enum Message {
    // TODO: define the different variants used below
    Move,
    Echo,
    Quit
}

impl Message {
    fn call(&self) {
        println!("{:?}", self);
    }
}

fn main() {
    let messages = [
        Message::Move { x: 10, y: 30 },
        Message::Echo(String::from("hello world")),
        Message::ChangeColor(200, 255, 255),
        Message::Quit,
    ];

    for message in &messages {
        message.call();
    }
}
