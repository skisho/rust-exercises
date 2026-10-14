// enums1.rs
//
// Define the `Message` enum so this program compiles. Notice that the
// variants carry different kinds of data.

// I AM NOT DONE

#[derive(Debug)]
enum Message {
    // TODO: define the variants used in `main`
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
