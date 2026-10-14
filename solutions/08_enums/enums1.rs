// enums1.rs - solution
//
// Enum variants can be unit-like, tuple-like, or struct-like.

#[derive(Debug)]
enum Message {
    Quit,
    Echo(String),
    Move { x: i32, y: i32 },
    ChangeColor(u8, u8, u8),
}

fn main() {
    let messages = [
        Message::Quit,
        Message::Echo(String::from("hello")),
        Message::Move { x: 10, y: -3 },
        Message::ChangeColor(255, 0, 128),
    ];

    for m in &messages {
        println!("{m:?}");
    }
}
