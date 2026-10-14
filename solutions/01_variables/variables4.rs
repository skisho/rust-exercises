// variables4.rs - solution
//
// "Shadowing": a second `let` with the same name creates a brand new
// variable, which may even have a different type. The old one is hidden.

fn main() {
    let input = "42";
    println!("the text is {input}");

    let input = input.parse::<i32>().unwrap();
    println!("the number doubled is {}", input * 2);
}
