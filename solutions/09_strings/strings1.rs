// strings1.rs - solution
//
// `"blue"` is a `&'static str` (a borrowed string slice), not an owned
// `String`. Convert with `.to_string()`, `String::from`, or `.to_owned()`.

fn current_favorite_color() -> String {
    "blue".to_string()
}

fn main() {
    let answer = current_favorite_color();
    println!("My current favorite color is {answer}");
}
